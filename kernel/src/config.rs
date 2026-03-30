use core::sync::atomic::{AtomicU8, AtomicU16};

pub struct AbyssConfig {
    pub atm_pressure: AtomicU16,
    pub gases: [(AtomicU8, AtomicU8); 5], // Gas mixtures for the dive (fO2, fHe)
    pub gf_high: AtomicU8, // Safety factor of algorithm; generally 0-2
    pub gf_low: AtomicU8,
    pub safety_stop: AtomicU8, // Safety stop time in minutes
    pub sample_rate: AtomicU16, // Sample rate of algorithm in miliseconds
    pub algorithm: AtomicU8
}

pub static CONFIG: AbyssConfig = AbyssConfig {
    atm_pressure: AtomicU16::new(845), // Scaled by 1000x. 1.013 bar = 1013. Wondergat should be 0.845 = 845

    gases: [
        (AtomicU8::new(21), AtomicU8::new(0)), // Air
        (AtomicU8::new(80), AtomicU8::new(0)), // EAN80
        (AtomicU8::new(0), AtomicU8::new(0)),
        (AtomicU8::new(0), AtomicU8::new(0)),
        (AtomicU8::new(0), AtomicU8::new(0))
    ],
    gf_high: AtomicU8::new(85),
    gf_low: AtomicU8::new(55),
    safety_stop: AtomicU8::new(3),
    sample_rate: AtomicU16::new(5000), // TODO: Change this back to 1000ms for production use
    algorithm: AtomicU8::new(0)
};