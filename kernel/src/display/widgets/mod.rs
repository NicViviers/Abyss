use crate::ffi::*;

mod label;
mod line;
mod bar;

pub use label::*;
pub use line::*;
pub use bar::*;

pub fn container(parent: *mut _lv_obj_t) -> *mut _lv_obj_t {
    unsafe {
        let cont = lv_obj_create(parent);
        
        lv_obj_set_size(cont, 466, 466);
        lv_obj_set_style_pad_left(cont, 0, 0);
        lv_obj_set_style_pad_right(cont, 0, 0);
        lv_obj_set_style_pad_top(cont, 0, 0);
        lv_obj_set_style_pad_bottom(cont, 0, 0);
        lv_obj_set_style_border_width(cont, 0, 0);
        lv_obj_set_style_bg_opa(cont, 0, 0); 
        lv_obj_set_style_shadow_width(cont, 0, 0);
        lv_obj_set_style_outline_width(cont, 0, 0);

        cont
    }
}