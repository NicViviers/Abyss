use crate::ffi::*;

pub fn line(
    parent: *mut _lv_obj_t,
    x: i16,
    y: i16,
    length: i16,
    horizontal: bool,
    colour: (u8, u8, u8),
    thickness: i16,
) -> *mut _lv_obj_t {
    unsafe {
        let obj = lv_obj_create(parent);

        if horizontal {
            lv_obj_set_width(obj, length);
            lv_obj_set_height(obj, thickness);
        } else {
            lv_obj_set_width(obj, thickness);
            lv_obj_set_height(obj, length);
        }

        lv_obj_set_pos(obj, x, y);

        let (r, g, b) = colour;
        let lv_col = lv_color_make(r, g, b);
        
        let selector = 0; 

        lv_obj_set_style_bg_color(obj, lv_col, selector);
        lv_obj_set_style_bg_opa(obj, 255, selector);
        lv_obj_set_style_border_width(obj, 0, selector);
        lv_obj_set_style_radius(obj, LV_RADIUS_CIRCLE as i16, selector);

        obj
    }
}