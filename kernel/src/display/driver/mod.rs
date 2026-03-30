use crate::{ffi::*, info};
use embedded_hal::delay::DelayNs;
use rp235x_hal::{Timer, timer::CopyableTimer0};

const FPS: u32 = 4;

pub struct Display {
    display_if: bsp_display_interface_t,
    pub screen: *mut lv_obj_t,
    timer: Timer<CopyableTimer0>,
    loop_delay: u32 // 1000 / FPS
}

impl Display {
    pub fn new(timer: Timer<CopyableTimer0>) -> Self {
        unsafe {
            info!("Initializing LVGL registers");

            bsp_i2c_init();
            lv_init();
            lv_port_disp_init(466, 466, 0, false);
            lv_port_indev_init(466, 466, 0);

            let scr = lv_scr_act();

            // Set background color to black
            lv_obj_set_style_bg_color(
                scr,
                lv_color_make(0, 0, 0),
                LV_PART_MAIN as u32,
            );

            // Make sure it's fully opaque
            lv_obj_set_style_bg_opa(
                scr,
                LV_OPA_COVER as u8,
                LV_PART_MAIN as u32,
            );

            Self {
                display_if: *display_if,
                screen: scr,
                timer,
                loop_delay: 1000 / FPS
            }
        }
    }

    // TODO: Can we update FPS on the fly or at least drop the display instance and create it with new fps?
    // Update on the fly will be a problem due to race condition across threads
    pub fn main_loop<F>(&mut self, mut sync: F) -> ! where F: FnMut() {
        info!("Render thread set to {} frames per second", 1000 / self.loop_delay);

        unsafe {
            loop {
                lv_tick_inc(self.loop_delay);
                sync();
                lv_timer_handler();
                self.timer.delay_ms(self.loop_delay); 
            }
        }
    }
}