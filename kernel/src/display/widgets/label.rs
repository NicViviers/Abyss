use heapless::String;
use crate::{ffi::*, info};

#[macro_export]
macro_rules! font {
    ( $f:ident ) => {
        unsafe { &crate::ffi::$f as *const crate::ffi::lv_font_t }
    }
}

#[derive(Debug)]
pub struct Label {
    obj: *mut _lv_obj_t,
    pub buffer: String<8>
}

impl Label {
    pub fn new(parent: *mut _lv_obj_t) -> Self {
        Self {
            obj: unsafe { lv_label_create(parent) },
            buffer: String::new()
        }
    }

    pub fn write_buffer(&mut self) {
        let _ = self.buffer.push('\0');
        
        unsafe {
            lv_label_set_text(self.obj, self.buffer.as_ptr() as *const u8);
            lv_obj_update_layout(self.obj);
        }
    }

    pub fn set_pos(&self, x: i16, y: i16) {
        unsafe { lv_obj_set_pos(self.obj, x, y); }
    }

    pub fn set_colour(&self, r: u8, g: u8, b: u8) {
        unsafe {
            lv_obj_set_style_text_color(
            self.obj,
            lv_color_make(r, g, b),
            LV_PART_MAIN as u32);
        }
    }

    pub fn set_font(&self, font: *const lv_font_t) {
        unsafe {
            lv_obj_set_style_text_font(self.obj, font, LV_STATE_DEFAULT);
        }
    }
}

pub fn label(parent: *mut _lv_obj_t, text: &'static str, x: i16, y: i16) -> Label {
    let mut inst = Label::new(parent);
    inst.buffer.push_str(text).unwrap();
    inst.write_buffer();
    inst.set_pos(x, y);
    inst.set_colour(255, 255, 255);

    inst
}