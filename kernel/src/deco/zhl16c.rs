use core::sync::atomic::Ordering;
use libm::ceilf;
use crate::{config::CONFIG, display::pages::DIVE_STATE, info};

static hN2: [f32; 16] = [
    5.0, 8.0, 12.5, 18.5,
    27.0, 38.3, 54.3, 77.0,
    109.0, 146.0, 187.0, 239.0,
    305.0, 390.0, 498.0, 635.0 
];

static hHe: [f32; 16] = [
    1.88, 3.02, 4.72, 6.99,
    10.21, 14.48, 20.53, 29.11,
    41.20, 55.19, 70.69, 90.34,
    115.29, 147.42, 188.24, 240.03 
];

static aN2: [f32; 16] = [
    1.1696, 1.0, 0.8618, 0.7562,
    0.62, 0.5043, 0.441, 0.4,
    0.375, 0.35, 0.3295, 0.3065,
    0.2835, 0.261, 0.248, 0.2327 
];

static aHe: [f32; 16] = [
    1.6189, 1.383, 1.1919, 1.0458,
    0.922, 0.8205, 0.7305, 0.6502,
    0.595, 0.5545, 0.5333, 0.5189,
    0.5181, 0.5176, 0.5172, 0.5119
];

static bN2: [f32; 16] = [
    0.5578, 0.6514, 0.7222, 0.7825,
    0.8126, 0.8434, 0.8693, 0.8910,
    0.9092, 0.9222, 0.9319, 0.9403,
    0.9477, 0.9544, 0.9602, 0.9653
];

static bHe: [f32; 16] = [
    0.4770, 0.5747, 0.6527, 0.7223,
    0.7582, 0.7957, 0.8279, 0.8553,
    0.8757, 0.8903, 0.8997, 0.9073,
    0.9122, 0.9171, 0.9217, 0.9267 
];

pub struct ZHL16C {
    n2_tissues: [f32; 16], // Tisue compartment loading
    he_tissues: [f32; 16],
    fo2: f32,
    fhe: f32,
    fn2: f32,
    atm_pressure: f32, // Atmospheric pressure
    sample_rate: f32,
    gf_high: f32,
    gf_low: f32,
    first_stop: Option<f32>,

    deco_stop_depth: f32, // Decompression stop depth
    deco_stop_time: f32, // Decompression stop time
    safety_stop_earned: bool,
    safety_stop_left: u32,
    last_depth: f32,
}

impl ZHL16C {
    pub fn new() -> Self {
        let atm_pressure = CONFIG.atm_pressure.load(Ordering::Acquire) as f32 / 1000.0;
        let sample_rate = (CONFIG.sample_rate.load(Ordering::Acquire) as f32 / 1000.0) / 60.0;

        let fo2 = CONFIG.gases[0].0.load(Ordering::Acquire) as f32 / 100.0;
        let fhe = CONFIG.gases[0].1.load(Ordering::Acquire) as f32 / 100.0;
        let fn2 = 1.0 - fo2 - fhe;

        let mut n2_tissues = [0.0; 16];
        let mut he_tissues = [0.0; 16];

        for i in 0 .. 16 {
            n2_tissues[i] = fn2 * (atm_pressure - 0.0627);
            he_tissues[i] = fhe * (atm_pressure - 0.0627);
        }

        Self {
            n2_tissues,
            he_tissues,
            fo2,
            fhe,
            fn2,
            atm_pressure,
            sample_rate,
            gf_high: CONFIG.gf_high.load(Ordering::Acquire) as f32 / 100.0,
            gf_low: CONFIG.gf_low.load(Ordering::Acquire) as f32 / 100.0,
            first_stop: None,

            deco_stop_depth: 0.0,
            deco_stop_time: 0.0,
            safety_stop_earned: false,
            safety_stop_left: 0,
            last_depth: 0.0
        }
    }

    // Refreshes tissue compartments at the current depth and time delta of CONFIG.sample_rate
    pub fn tick(&mut self) {
        // Refresh current gas mixture (The UI handles user interactions)
        let active_gas = DIVE_STATE.gas.load(Ordering::Acquire) as usize;
        let new_o2 = CONFIG.gases[active_gas].0.load(Ordering::Acquire) as f32 / 100.0;
        let new_he = CONFIG.gases[active_gas].1.load(Ordering::Acquire) as f32 / 100.0;

        if new_o2 != self.fo2 || new_he != self.fhe {
            self.fo2 = new_o2;
            self.fhe = new_he;
            self.fn2 = 1.0 - new_o2 - new_he;
            info!("Gas changed to: {}/{}", self.fo2 * 100.0, self.fhe * 100.0);
        }

        // Continue with the decompression calculations
        let current_depth = DIVE_STATE.depth_scaled.load(Ordering::Acquire) as f32 / 10.0;

        // Calculate ascent rate
        let depth_delta = self.last_depth - current_depth;
        let ascent_rate = depth_delta / self.sample_rate;

        // Map the rate to 0-5 scale
        let widget_value: i8 = if ascent_rate <= 1.0 {
            0 // All icons off
        } else if ascent_rate <= 3.0 {
            1 // 1 Bar visible
        } else if ascent_rate <= 6.0 {
            2 // 2 Bars
        } else if ascent_rate <= 8.0 {
            3 // 3 Bars
        } else if ascent_rate <= 10.0 {
            4 // 4 Bars (Caution)
        } else {
            5 // 5 Bars (Violation / Red)
        };

        DIVE_STATE.ascent.store(widget_value, Ordering::Release);
        self.last_depth = current_depth; // Update last depth for next tick() call

        // The diver "earns" the safety stop if they exceed a depth of 10 meters even once
        if current_depth >= 10.0 {
            self.safety_stop_earned = true;

            if self.safety_stop_left == 0 {
                self.safety_stop_left = CONFIG.safety_stop.load(Ordering::Acquire) as u32 * 60 * 1000;
            }
        }

        // Calculate p_gas (Accounting for water vapour pressure of 0.0627)
        let p_amb = self.atm_pressure + (current_depth / 10.0);
        let p_gas_n2 = (p_amb - 0.0627) * self.fn2;
        let p_gas_he = (p_amb - 0.0627) * self.fhe;

        for i in 0 .. 16 {
            self.n2_tissues[i] = ZHL16C::comp_loading(
                self.n2_tissues[i],
                p_gas_n2,
                self.sample_rate,
                hN2[i]
            );

            self.he_tissues[i] = ZHL16C::comp_loading(
                self.he_tissues[i],
                p_gas_he,
                self.sample_rate,
                hHe[i]
            );
        }

        // How much time is theoretically left at this stop
        let mandatory_ceiling = self.calculate_first_stop_depth();
        let mut minutes_at_stop = 0.0;
        if mandatory_ceiling >= 3.0 {
            self.deco_stop_depth = mandatory_ceiling;
            minutes_at_stop = self.calculate_stop_time();
        }

        if mandatory_ceiling > 3.0 || (mandatory_ceiling == 3.0 && minutes_at_stop > 0.1) {
            if self.first_stop.is_none() || mandatory_ceiling > self.first_stop.unwrap() {
                self.first_stop = Some(mandatory_ceiling);
            }

            // Calculate time at the current stop depth
            let mut minutes_left = minutes_at_stop;
            let mut display_depth = self.deco_stop_depth;

            // If current stop is cleared (<= 0.1 min) and we aren't at the final stop (3m),
            // show the diver the NEXT stop depth and how long they'll be there
            if minutes_left <= 0.1 && mandatory_ceiling > 3.0 {
                display_depth = mandatory_ceiling - 3.0;
                
                // Temporarily swap depth to predict the time at the next shallower level
                let current_actual = self.deco_stop_depth;
                self.deco_stop_depth = display_depth;
                minutes_left = self.calculate_stop_time();
                self.deco_stop_depth = current_actual; // Revert state
            }

            // Update UI State
            DIVE_STATE.deco_prefix.store(if self.deco_stop_depth > 12.0 {
                2
            } else { 3 }, Ordering::Release);
            DIVE_STATE.deco_depth_scaled.store((display_depth * 10.0) as i16, Ordering::Release);
            DIVE_STATE.deco_time.store(ceilf(minutes_left) as i16, Ordering::Release);

            
            
            info!("\tDisplay: {:.1} min @ {}m (Actual Ceiling: {})", minutes_left, display_depth, mandatory_ceiling);
            
            let stop_mins = CONFIG.safety_stop.load(Ordering::Acquire) as u32;
            self.safety_stop_left = stop_mins * 60 * 1000;
        } else {
            // Calculate NDL since we don't have a mandatory ceiling
            let ndl = self.calculate_ndl(current_depth);
            
            self.deco_stop_depth = 0.0;
            // self.first_stop = None;

            // Check if we are in the Safety Stop window
            let in_safety_zone = current_depth <= 6.0 && current_depth >= 2.0;

            if self.safety_stop_earned && in_safety_zone {
                DIVE_STATE.deco_prefix.store(2, Ordering::Release); // Signal "SAFETY STOP"
                DIVE_STATE.deco_depth_scaled.store(30, Ordering::Release);

                let sample_ms = CONFIG.sample_rate.load(Ordering::Acquire) as u32;
                if let Some(time_left) = self.safety_stop_left.checked_sub(sample_ms) {
                    self.safety_stop_left = time_left;
                } else {
                    self.safety_stop_left = 0;
                    self.safety_stop_earned = false;
                }
                
                let display_mins = libm::ceilf(self.safety_stop_left as f32 / 60000.0) as i16;
                DIVE_STATE.deco_time.store(display_mins, Ordering::Release);
                info!("\tSafety Stop: {}ms left", self.safety_stop_left);
            } else {
                // --- NO DECO (NDL) ---
                DIVE_STATE.deco_prefix.store(0, Ordering::Release); // Signal "NO DECO"
                DIVE_STATE.deco_time.store(ndl as i16, Ordering::Release);
                info!("\tNDL: {:.1}", ndl);
            }
        }
    }

    fn calculate_first_stop_depth(&self) -> f32 {
        let mut max_p_ceiling = self.atm_pressure;

        for i in 0..16 {
            let p_total = self.n2_tissues[i] + self.he_tissues[i];
            if p_total <= self.atm_pressure { continue; }

            // 1. Blend coefficients
            let a = (aN2[i] * self.n2_tissues[i] + aHe[i] * self.he_tissues[i]) / p_total;
            let b = (bN2[i] * self.n2_tissues[i] + bHe[i] * self.he_tissues[i]) / p_total;

            // 2. Solve for Ceiling Pressure at GF_LOW
            let b_adj = 1.0 / b - 1.0;
            let p_ceiling = (p_total - a * self.gf_low) / (self.gf_low * b_adj + 1.0);

            if p_ceiling > max_p_ceiling {
                max_p_ceiling = p_ceiling;
            }
        }

        // Convert Ceiling (Bar) -> Meters (Relative to Wondergat/Local Atm)
        let ceiling_m = (max_p_ceiling - self.atm_pressure) * 10.0;

        if ceiling_m <= 0.0 {
            0.0
        } else {
            // Round up to the nearest 3m stop (industry standard)
            info!("\tCeiling at {}m", libm::ceilf(ceiling_m / 3.0) * 3.0);
            libm::ceilf(ceiling_m / 3.0) * 3.0
        }
    }

    fn get_gf(&self, current_depth: f32) -> f32 {
        match self.first_stop {
            // If we have a deco obligation
            Some(first_stop) if first_stop > 0.0 => {
                if current_depth >= first_stop {
                    self.gf_low
                } else if current_depth <= 0.0 {
                    self.gf_high
                } else {
                    // Linear Interpolation: Move from GF_low at first_stop 
                    // to GF_high at the surface.
                    self.gf_high + (self.gf_low - self.gf_high) * (current_depth / first_stop)
                }
            },
            // No deco obligation yet? Just use GF_low for NDL safety.
            _ => self.gf_low,
        }
    }

    pub fn calculate_ndl(&self, current_depth: f32) -> f32 {
        let p_amb = self.atm_pressure + (current_depth / 10.0);
        let p_wv = 0.0627; // Water vapour pressure
        
        let p_gas_n2 = (p_amb - p_wv) * self.fn2;
        let p_gas_he = (p_amb - p_wv) * self.fhe;
        let p_gas_total = p_gas_n2 + p_gas_he;

        let mut min_ndl = 999.0; // Start with a high value

        for i in 0..16 {
            let p_begin_total = self.n2_tissues[i] + self.he_tissues[i];

            // Calculate weighted coefficients based on current tissue ratio
            let a = (aN2[i] * self.n2_tissues[i] + aHe[i] * self.he_tissues[i]) / p_begin_total;
            let b = (bN2[i] * self.n2_tissues[i] + bHe[i] * self.he_tissues[i]) / p_begin_total;

            // Calculate the maximum pressure this tissue can hold at the surface
            // This is the M-Value formula adjusted for GF_high
            let p_limit = self.atm_pressure + self.gf_high * (self.atm_pressure / b + a - self.atm_pressure);

            // If the gas we are breathing is lower than the limit, we'll never hit NDL
            if p_gas_total <= p_limit {
                continue;
            }
            
            // If we are already past the limit, NDL is 0
            if p_begin_total >= p_limit {
                return 0.0;
            }

            // Solve for time (t)
            // Using a weighted half-time for the N2/He mix
            let half_time = (hN2[i] * self.n2_tissues[i] + hHe[i] * self.he_tissues[i]) / p_begin_total;
            let time_to_limit = - (half_time / 0.693147) * libm::logf((p_gas_total - p_limit) / (p_gas_total - p_begin_total));

            if time_to_limit < min_ndl {
                min_ndl = time_to_limit;
            }
        }

        if min_ndl > 99.0 { 99.0 } else { min_ndl }
    }

    // Calculates in minutes how long the current decompression stop will take
    fn calculate_stop_time(&self) -> f32 {
        if self.deco_stop_depth <= 0.0 { return 0.0; }

        let target_depth = self.deco_stop_depth - 3.0;
        let p_amb_target = self.atm_pressure + (target_depth / 10.0);
        let gf_current_stop = self.get_gf(self.deco_stop_depth);
        let p_wv = 0.0627; // Water vapour pressure constant

        // To calculate "Time at Stop" while still at the bottom, 
        // we must assume the diver is already AT the stop depth (e.g., 6m).
        // Otherwise, p_gas remains too high and the math thinks we are still loading.
        let p_amb_at_stop = self.atm_pressure + (self.deco_stop_depth / 10.0);
        let p_gas_at_stop = (p_amb_at_stop - p_wv) * (self.fn2 + self.fhe);

        let mut max_time_at_stop = 0.0;

        for i in 0..16 {
            let p_begin_total = self.n2_tissues[i] + self.he_tissues[i];
            
            // Calculate weighted coefficients (a & b) according to the current mixture
            // N2 and He have different diffusion rates so we need to blend them
            let a = (aN2[i] * self.n2_tissues[i] + aHe[i] * self.he_tissues[i]) / p_begin_total;
            let b = (bN2[i] * self.n2_tissues[i] + bHe[i] * self.he_tissues[i]) / p_begin_total;

            // The maximum pressure the tissue is allowed to have before it can move to the target depth
            // This is the M-Value formula solving for allowed tissue pressure
            let p_limit_at_target = p_amb_target * (gf_current_stop / b - gf_current_stop + 1.0) + a * gf_current_stop;

            // If this tissue is already clear to move to the next stop, skip it
            if p_begin_total <= p_limit_at_target { continue; }

            // Weighted half-time for the mix (accounts for Helium's faster movement)
            let half_time = (hN2[i] * self.n2_tissues[i] + hHe[i] * self.he_tissues[i]) / p_begin_total;

            // Determine the pressure difference for the log calculation
            let diff_top = p_gas_at_stop - p_limit_at_target;
            let diff_bot = p_gas_at_stop - p_begin_total;

            // Solve for time (t) using the inverse Schreiner Equation
            // We only calculate if we are off-gassing (diff_top and diff_bot are both negative)
            if diff_top < 0.0 && diff_bot < 0.0 {
                // Natural logarithm using libm for RP2350 hardware FPU support
                let time = - (half_time / 0.693147) * libm::logf(diff_top / diff_bot);
                
                // The total stop time is governed by the slowest (limiting) compartment
                if time > max_time_at_stop { 
                    max_time_at_stop = time; 
                }
            }
        }
        max_time_at_stop
    }

    // TODO: Implement CNS
    // fn comp_cns(&mut self, ppo2: f32) {
    //     // We only care about oxygen debt if PPO2 is above 0.5 bar
    //     if ppo2 <= 0.5 { return; }

    //     // Math-based curve fit for the NOAA table:
    //     // This curve closely approximates the 0.6 - 1.6 bar limits.
    //     // At 1.6, it gives ~45 mins. At 1.0, it gives ~300 mins.
    //     let limit = 280.0 * libm::powf(ppo2, -4.65);

    //     // Calculate gain for this sample (sample_rate is in minutes)
    //     let gain = (self.sample_rate / limit) * 100.0;
    //     self.cns += gain;

    //     // Safety clamps
    //     if self.cns > 255.0 { self.cns = 255.0; }
        
    //     // Release to UI
    //     DIVE_STATE.cns.store(self.cns as u8, Ordering::Release);
    // }

    // Computes the loading (bar) of a tissue compartment at a depth (bar) for a given time (minutes)
    fn comp_loading(p_begin: f32, p_gas: f32, te: f32, tht: f32) -> f32 {
        let k = 0.693147 / tht;
        p_begin + (p_gas - p_begin) * (1.0 - libm::expf(-k * te))
    }
}