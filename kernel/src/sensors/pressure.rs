use crate::info;

pub struct PressureSensor {
    depth: [i16; 406],
    temp: [u8; 460],
    pub i: usize
}

impl PressureSensor {
    pub fn new() -> Self {
        Self {
            depth: [
                // --- DESCENT: 0m to 70m (approx 2.8m per 5s sample) ---
                0, 28, 56, 84, 112, 140, 168, 196, 224, 252, 280, 308, 336, 364, 392, 420, 448, 476, 504, 532, 560, 588, 616, 644, 672, 700,

                // --- PPO2 ALARM TEST: 70m (700dm) for 1 minute (12 samples) ---
                // At 70m, PPO2 = (7.0 + 1.0) * 0.21 = 1.68 bar. 
                700, 700, 700, 700, 700, 700, 700, 700, 700, 700, 700, 700,

                // --- ASCENT TO MAIN BOTTOM: 70m to 55m (approx 3 samples) ---
                650, 600, 550,

                // --- BOTTOM TIME: 55m (550dm) for 8 minutes (96 samples) ---
                // This will build a significant deco obligation.
                550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550,
                550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550,
                550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550,
                550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550,
                550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550,
                550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550, 550,

                // --- THE CUSTOM ASCENT (Starts around Index 136) ---
                // 55m to 15m (10m/min)
                550, 542, 533, 525, 517, 508, 500, 492, 483, 475, 467, 458, 450, 442, 433, 425, 
                417, 408, 400, 392, 383, 375, 367, 358, 350, 342, 333, 325, 317, 308, 300, 292, 
                283, 275, 267, 258, 250, 242, 233, 225, 217, 208, 200, 192, 183, 175, 167, 158, 
                150, // Index 184 (15.0m)

                // --- INDEX 185: Stop at 15m for 30 seconds (6 samples) ---
                150, 150, 150, 150, 150, 150, 

                // --- DESCEND to 12m for 1.5 minutes (18 samples) ---
                // Transition 15m -> 12m
                142, 134, 126, 123,
                // At 12m (Indices 194 to 211)
                120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120, 120,
                120, 120, 120, 120, 130,

                // --- ASCEND to 9m ---
                110, 100, 90, // Index 214 is the first hit at 9.0m

                // --- Index 215+: Final Deco / Gas Change Window ---
                90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90, 90,

                82, 74, 68, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60,
                60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60, 60,
                60, 60, 60, 60, 60, 60, 60, 60, 60,
                52, 44, 36, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                // Safety stop
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
                30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30, 30,
            ],

            temp: {
                const TOTAL_SAMPLES: usize = 460;
                let mut temp_readings = [0u8; TOTAL_SAMPLES];

                // Different seed than the depth calculation to avoid identical "noise" patterns
                let mut seed: u32 = 99; 

                for t in 0..TOTAL_SAMPLES {
                    // LCG algorithm to generate a pseudo-random number
                    seed = seed.wrapping_mul(1103515245).wrapping_add(12345);
                    
                    // Scale the random number to a range of 0 to 6, then add 14
                    // (seed % 7) gives 0, 1, 2, 3, 4, 5, or 6
                    let random_temp = (seed % 7) as u8;
                    temp_readings[t] = 14 + random_temp;
                }

                temp_readings
            },

            i: 0
        }
    }

    // This must returned scaled depth, so 50m = 500
    pub fn read(&mut self) -> (i16, u8) {
        // let depth = self.depth[self.i];
        // let temp = self.temp[self.i];
        // info!("\tDepth index: {} ({}m)", self.i, depth);
        // self.i += 1;
        // (depth, temp)
        (0, 0)
    }
}