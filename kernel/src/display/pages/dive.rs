use crate::config::CONFIG;
use crate::{ffi::*, info};
use crate::display::widgets::*;
use crate::font;
use core::sync::atomic::{AtomicI16, AtomicU16, AtomicI8, AtomicBool, Ordering};
use core::fmt::Write;

const LINE_COLOUR: (u8, u8, u8) = (8, 16, 8);

pub struct SharedDiveState {
    pub depth_scaled: AtomicI16,
    pub temp: AtomicI16,

    pub ascent: AtomicI8,
    pub deco_prefix: AtomicI8, // 0 = 'NO', 1 = 'DEEP', 2 = 'SAFE'
    pub deco_depth_scaled: AtomicI16,
    pub deco_time: AtomicI16,
    
    pub dive_time: AtomicU16, // Scaled by 10x meaning 16.2 minutes = 162
    pub gas: AtomicI8,
    pub gas_mode: AtomicI8, // 0 = 'EAN%', 1 = 'CNS%'

    pub is_aod: AtomicBool,
}

// Global instance
pub static DIVE_STATE: SharedDiveState = SharedDiveState {
    depth_scaled: AtomicI16::new(0),
    temp: AtomicI16::new(0),

    ascent: AtomicI8::new(0),
    deco_prefix: AtomicI8::new(0), // 0 = 'NO', 1 = 'DEEP', 2 = 'SAFE'
    deco_depth_scaled: AtomicI16::new(0),
    deco_time: AtomicI16::new(0),

    dive_time: AtomicU16::new(0),
    gas: AtomicI8::new(0),
    gas_mode: AtomicI8::new(0), // 0 = 'EAN%', 1 = 'CNS%'

    is_aod: AtomicBool::new(false)
};

pub struct DiveUI {
    pub nonvital: *mut _lv_obj_t, // Pointer to non vital data container
    pub depth: Label,
    pub temp: Label,
    pub ascent: Bar<5>,
    pub deco_prefix: Label,
    pub deco_depth: Label,
    pub deco_time: Label,
    pub dive_time: Label,
    pub ppo2: Label,
    pub ppn2: Label,
    pub gas: Label,
    pub o2_perc: Label,
    pub o2_setting: Label
}

impl DiveUI {
    pub fn set_aod(&mut self, is_aod: bool) {
        // Non-essential data is hidden
        unsafe {
            if is_aod {
                lv_obj_add_flag(self.nonvital, LV_OBJ_FLAG_HIDDEN);
            } else {
                lv_obj_clear_flag(self.nonvital, LV_OBJ_FLAG_HIDDEN);
            }
        }
    }

    pub fn sync(&mut self) {
        let ordering = Ordering::Acquire;

        // Depth
        let d_raw = DIVE_STATE.depth_scaled.load(ordering);
        let cur_depth = d_raw as f32 / 10.0;
        self.depth.buffer.clear();
        let _ = write!(self.depth.buffer, "{}.{}m", d_raw / 10, d_raw % 10);
        self.depth.write_buffer();

        // Temperature
        let t_raw = DIVE_STATE.temp.load(ordering);
        self.temp.buffer.clear();
        let _ = write!(self.temp.buffer, "{}°", t_raw);
        self.temp.write_buffer();

        // Ascent
        let a_raw = DIVE_STATE.ascent.load(ordering);
        self.ascent.set_value(a_raw as u8);

        // Deco prefix
        self.deco_prefix.buffer.clear();
        let mut prefix_pos = (100, 125);
        let mut prefix_colour = (255, 255, 255);
        match DIVE_STATE.deco_prefix.load(ordering) {
            0 => self.deco_prefix.buffer.push_str("NO").unwrap(),
            1 => {
                self.deco_prefix.buffer.push_str("DEEP").unwrap();
                prefix_pos = (80, 125);
            }

            2 => {
                self.deco_prefix.buffer.push_str("SAFE").unwrap();
                prefix_pos = (80, 125);
            }

            3 => { // Normal deco
                prefix_colour = LINE_COLOUR;
            }

            _ => {}
        }
        self.deco_prefix.set_colour(prefix_colour.0, prefix_colour.1, prefix_colour.2);
        self.deco_prefix.set_pos(prefix_pos.0, prefix_pos.1);
        self.deco_prefix.write_buffer();

        // Deco depth
        let d_raw = DIVE_STATE.deco_depth_scaled.load(ordering);
        self.deco_depth.buffer.clear();
        if !self.deco_prefix.buffer.contains("NO") {
            write!(self.deco_depth.buffer, "{}m", d_raw / 10).unwrap();
        }
        self.deco_depth.write_buffer();

        // Deco time
        let d_raw = DIVE_STATE.deco_time.load(ordering);
        self.deco_time.buffer.clear();
        let _ = write!(self.deco_time.buffer, "{}", d_raw);
        self.deco_time.write_buffer();

        // Deco prefix
        let s_raw = DIVE_STATE.deco_prefix.load(ordering);
        self.deco_prefix.buffer.clear();
        let _ = match s_raw {
            0 => self.deco_prefix.buffer.push_str("NO"),
            1 => self.deco_prefix.buffer.push_str("DEEP"),
            2 => self.deco_prefix.buffer.push_str("SAFE"),
            _ => Ok(())
        };
        self.deco_prefix.write_buffer();

        // Dive time
        let d_raw = DIVE_STATE.dive_time.load(ordering);
        self.dive_time.buffer.clear();
        let _ = write!(self.dive_time.buffer, "{}", d_raw / 100);
        self.dive_time.write_buffer();

        // Selected gas & value
        let sel_gas = DIVE_STATE.gas.load(ordering);
        self.gas.buffer.clear();
        let _ = write!(self.gas.buffer, "G{}", sel_gas + 1);
        self.gas.write_buffer();

        let g_raw = DIVE_STATE.gas_mode.load(ordering);
        self.o2_perc.buffer.clear();
        self.o2_setting.buffer.clear();
        match g_raw {
            0 => {
                let current_gas = &CONFIG.gases[DIVE_STATE.gas.load(Ordering::Acquire) as usize];
                let current_fo2 = current_gas.0.load(Ordering::Acquire);
                let current_fhe = current_gas.1.load(Ordering::Acquire);

                if current_fhe != 0 {
                    write!(self.o2_perc.buffer, "{}/{}", current_fo2, current_fhe).unwrap();
                    self.o2_setting.buffer.write_str("TX%").unwrap();
                    self.o2_perc.set_pos(290, 270);
                } else {
                    write!(self.o2_perc.buffer, "{}", current_fo2).unwrap();
                    self.o2_setting.buffer.write_str("EAN%").unwrap();
                    self.o2_perc.set_pos(350, 270);
                }
            }

            1 => {
                // TODO: Calculate CNS% loading here
                write!(self.o2_perc.buffer, "00").unwrap();
                write!(self.o2_setting.buffer, "CNS%").unwrap();
            }

            _ => {}
        }
        self.o2_perc.write_buffer();
        self.o2_setting.write_buffer();

        // PPO2
        self.ppo2.buffer.clear();
        let depth_dm: i32 = (cur_depth * 10.0) as i32;
        let ata_10: i32 = depth_dm / 10 + 10;
        let fo2_100: i32 = CONFIG.gases[sel_gas as usize].0.load(ordering) as i32;
        let ppo2_100: i32 = (ata_10 * fo2_100 + 5) / 10;
        let int_part = ppo2_100 / 100;
        let frac_part = (ppo2_100 % 100).abs();

        // Warn on PPO2 > 1.6 and PPO2 < 0.18
        if ((int_part >= 1 && frac_part > 60) || int_part >= 2) || (int_part == 0 && frac_part < 18) {
            self.ppo2.set_colour(255, 0, 0);
        } else {
            self.ppo2.set_colour(255, 255, 255);
        }

        let _ = write!(
            self.ppo2.buffer,
            "{}.{:02}",
            int_part,
            frac_part
        );
        self.ppo2.write_buffer();

        // PPN2
        // NOTE: This does not account for helium mixtures
        self.ppn2.buffer.clear();
        let depth_dm: i32 = (cur_depth * 10.0) as i32;
        let ata_10: i32 = depth_dm / 10 + 10;
        let fn2_100: i32 = 100 - fo2_100;
        let ppn2_10: i32 = (ata_10 * fn2_100 + 50) / 100;
        let int_part = ppn2_10 / 10;
        let frac_part = (ppn2_10 % 10).abs();

        let _ = write!(
            self.ppn2.buffer,
            "{}.{}",
            int_part,
            frac_part
        );
        self.ppn2.write_buffer();

        // TODO: Implement gas switching with physical button (must be UI logic)

        let aod_requested = DIVE_STATE.is_aod.load(ordering);
        self.set_aod(aod_requested);
    }
}

pub fn dive(screen: *mut _lv_obj_t) -> DiveUI {
    let nonvital = container(screen);

    // Depth
    label(screen, "DEPTH", 145, 20).set_font(font!(lv_font_montserrat_24));
    let depth_txt = label(screen, "100.0m", 100, 50);
    depth_txt.set_font(font!(lv_font_montserrat_44));

    // Temperature
    label(nonvital, "TEMP", 260, 20).set_font(font!(lv_font_montserrat_24));
    let temp_txt = label(nonvital, "24°", 300, 56);
    temp_txt.set_font(font!(lv_font_montserrat_36));

    // Separator
    line(screen, 0, 100, 466, true, LINE_COLOUR, 5);

    // Ascent indicator
    let ascent = bar::<5>(
        screen,
        -1,
        20,
        110,
        "^",
        font!(lv_font_montserrat_48),
        -10,
        (255, 255, 255),
        LINE_COLOUR
    );
    label(screen, "ASC", 13, 218);

    // Deco
    // deco_prefix_txt = 'NO' | 'DEEP' | 'SAFE'
    let deco_prefix_txt = label(screen, "NO", 100, 125);
    deco_prefix_txt.set_font(font!(lv_font_montserrat_24));
    label(screen, "DECO", 150, 125).set_font(font!(lv_font_montserrat_24));

    let deco_depth_txt = label(screen, "12m", 80, 170);
    deco_depth_txt.set_font(font!(lv_font_montserrat_48));

    let deco_time_txt = label(screen, "30", 204, 170);
    deco_time_txt.set_font(font!(lv_font_montserrat_48));

    line(screen, 0, 250, 466, true, LINE_COLOUR, 5);

    // Dive time
    label(screen, "DIVE", 15, 278);
    label(screen, "TIME", 15, 293);
    let dive_time_txt = label(screen, "0", 60, 270);
    dive_time_txt.set_font(font!(lv_font_montserrat_44));

    // TODO: G1 | G2 | G3 (fO2 or fO2/fHe) && CNS% (Flash while selecting, show PPO2 when selecting otherwise rotate?)
    let mut o2_perc_txt = label(nonvital, "21", 355, 270); // EANxy | CNSxy
    o2_perc_txt.set_font(font!(lv_font_montserrat_44));

    let mut o2_setting_txt = label(nonvital, "EAN%", 405, 275); // EAN% | CNS%
    o2_setting_txt.set_font(font!(lv_font_montserrat_16));

    let gas = label(nonvital, "G1", 405, 290); // G1 | G2 | G3
    gas.set_font(font!(lv_font_montserrat_24));

    let current_gas = &CONFIG.gases[DIVE_STATE.gas.load(Ordering::Acquire) as usize];
    let current_fo2 = current_gas.0.load(Ordering::Acquire);
    let current_fhe = current_gas.1.load(Ordering::Acquire);

    if current_fhe != 0 {
        o2_perc_txt.buffer.clear();
        write!(o2_perc_txt.buffer, "{}/{}", current_fo2, current_fhe).unwrap();
        o2_perc_txt.write_buffer();
        o2_perc_txt.set_pos(290, 270);

        o2_setting_txt.buffer.clear();
        o2_setting_txt.buffer.write_str("TX%").unwrap();
        o2_setting_txt.write_buffer();
    }

    // PPO2
    label(nonvital, "PP", 130, 348);
    label(nonvital, "O2", 130, 363);
    let ppo2_txt = label(nonvital, "0.21", 160, 340);
    ppo2_txt.set_font(font!(lv_font_montserrat_44));

    // PPN2
    label(nonvital, "PP", 260, 348);
    label(nonvital, "N2", 260, 363);
    let ppn2_txt = label(nonvital, "0.8", 290, 340);
    ppn2_txt.set_font(font!(lv_font_montserrat_44));

    // TODO: Extra info like battery voltage, do we have compass details etc?
    DiveUI {
        nonvital: nonvital,
        depth: depth_txt,
        temp: temp_txt,
        ascent,

        deco_prefix: deco_prefix_txt,
        deco_depth: deco_depth_txt,
        deco_time: deco_time_txt,

        dive_time: dive_time_txt,
        gas: gas,
        o2_perc: o2_perc_txt,
        o2_setting: o2_setting_txt,

        ppo2: ppo2_txt,
        ppn2: ppn2_txt
    }
}