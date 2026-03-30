mod zhl16c;
mod wkpp;

use core::sync::atomic::Ordering;
use embedded_hal::delay::DelayNs;
use rp235x_pac::SIO;
use rp235x_hal as hal;
use hal::timer::{Timer, CopyableTimer1};

pub use zhl16c::*;
pub use wkpp::*;

use crate::{config::CONFIG, display::pages::DIVE_STATE, info, sensors::PressureSensor};

enum ActiveAlgorithm {
    ZHL16C(ZHL16C),
    WKPP(WKPP)
}

pub fn dive_loop() {
    info!("Core 2 initialized");
    let sio = unsafe { &*SIO::ptr() };
    let raw_ptr = sio.fifo_rd().read().bits();
    let timer: &mut Timer<CopyableTimer1> = unsafe { &mut *(raw_ptr as *mut Timer<CopyableTimer1>) };
    info!("Core 2 timer received");

    let sample_rate = CONFIG.sample_rate.load(Ordering::Relaxed);
    let mut algorithm = match CONFIG.algorithm.load(Ordering::Relaxed) {
        0 => ActiveAlgorithm::ZHL16C(ZHL16C::new()),
        1 => ActiveAlgorithm::WKPP(WKPP::new()),
        _ => unreachable!()
    };

    info!("Sample rate: {}", sample_rate);

    let mut pressure_sensor = PressureSensor::new();

    info!("Pressure sensor simulation started");

    loop {
        timer.delay_ms(sample_rate as u32 / 5);

        // Store dive time
        DIVE_STATE.dive_time.fetch_add(sample_rate / 1000, Ordering::Release);

        // Store depth and temperature
        let (depth, temp) = pressure_sensor.read();
        DIVE_STATE.depth_scaled.store(depth, Ordering::Release);
        DIVE_STATE.temp.store(temp as i16, Ordering::Release);

        if pressure_sensor.i == 221 {
            DIVE_STATE.gas.store(1, Ordering::Release);
        }

        match algorithm {
            ActiveAlgorithm::ZHL16C(ref mut a) => a.tick(),
            _ => unimplemented!()
        };
    }
}