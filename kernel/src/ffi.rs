#![allow(dead_code)]
#![allow(non_upper_case_globals)]
#![allow(non_camel_case_types)]
#![allow(non_snake_case)]
include!(concat!(env!("OUT_DIR"), "/bindings.rs"));

//pub type LvCoord = i32;
pub type LvCoord = lv_coord_t;

// #define _LV_COORD_TYPE(x)       ((x) & _LV_COORD_TYPE_MASK)  /*Extract type specifiers*/
#[inline(always)]
pub const fn _LV_COORD_TYPE(x: lv_coord_t) -> u16 {
    (x as u16) & (_LV_COORD_TYPE_MASK as u16)
}

// #define _LV_COORD_PLAIN(x)      ((x) & ~_LV_COORD_TYPE_MASK) /*Remove type specifiers*/
#[inline(always)]
pub const fn _LV_COORD_PLAIN(x: lv_coord_t) -> u16 {
    (x as u16) & !(_LV_COORD_TYPE_MASK as u16)
}

// #define LV_COORD_IS_PX(x)       (_LV_COORD_TYPE(x) == _LV_COORD_TYPE_PX || \
//                                  _LV_COORD_TYPE(x) == _LV_COORD_TYPE_PX_NEG ? true : false)
#[inline(always)]
pub const unsafe fn LV_COORD_IS_PX(x: LvCoord) -> bool {
    let coord_type = _LV_COORD_TYPE(x);

    coord_type == _LV_COORD_TYPE_PX as u16 || coord_type == _LV_COORD_TYPE_PX_NEG as u16
}

// #define LV_COORD_IS_SPEC(x)     (_LV_COORD_TYPE(x) == _LV_COORD_TYPE_SPEC ? true : false)
#[inline(always)]
pub const unsafe fn LV_COORD_IS_SPEC(x: LvCoord) -> bool {
    _LV_COORD_TYPE(x) == _LV_COORD_TYPE_SPEC as u16
}

// #define LV_COORD_SET_SPEC(x)    ((x) | _LV_COORD_TYPE_SPEC)
#[inline(always)]
pub const unsafe fn LV_COORD_SET_SPEC(x: LvCoord) -> LvCoord {
    ((x as u16) | _LV_COORD_TYPE_SPEC as u16) as LvCoord
}

// /*Special coordinates*/
// #define LV_PCT(x)               (x < 0 ? LV_COORD_SET_SPEC(1000 - (x)) : LV_COORD_SET_SPEC(x))
#[inline(always)]
pub const unsafe fn LV_PCT(x: LvCoord) -> LvCoord {
    if x < 0 {
        LV_COORD_SET_SPEC(1000 - x)
    } else {
        LV_COORD_SET_SPEC(x)
    }
}

// #define LV_COORD_IS_PCT(x)      ((LV_COORD_IS_SPEC(x) && _LV_COORD_PLAIN(x) <= 2000) ? true : false)
#[inline(always)]
pub const unsafe fn LV_COORD_IS_PCT(x: LvCoord) -> bool {
    LV_COORD_IS_SPEC(x) && _LV_COORD_PLAIN(x) <= 2000
}

// #define LV_COORD_GET_PCT(x)     (_LV_COORD_PLAIN(x) > 1000 ? 1000 - _LV_COORD_PLAIN(x) : _LV_COORD_PLAIN(x))
#[inline(always)]
pub const unsafe fn LV_COORD_GET_PCT(x: LvCoord) -> LvCoord {
    if _LV_COORD_PLAIN(x) > 1000 {
        1000 - _LV_COORD_PLAIN(x) as LvCoord
    } else {
        _LV_COORD_PLAIN(x) as LvCoord
    }
}

// #define LV_SIZE_CONTENT         LV_COORD_SET_SPEC(2001)
#[inline(always)]
pub const unsafe fn LV_SIZE_CONTENT() -> LvCoord {
    LV_COORD_SET_SPEC(2001)
}

#[inline(always)]
pub unsafe fn lv_scr_act() -> *mut _lv_obj_t {
    lv_disp_get_scr_act(lv_disp_get_default())
}

#[inline(always)]
pub const unsafe fn lv_pct(x: LvCoord) -> LvCoord {
    LV_PCT(x)
} 

#[inline(always)]
pub unsafe fn lv_slider_set_range(obj: *mut lv_obj_t, min: i32, max: i32) {
    lv_bar_set_range(obj, min, max);
}

#[inline(always)]
pub unsafe fn lv_slider_set_value(obj: *mut lv_obj_t, value: i32, anim: lv_anim_enable_t) {
    lv_bar_set_value(obj, value, anim);
}

#[inline(always)]
pub unsafe fn lv_slider_get_value(obj: *mut lv_obj_t) -> i32 {
    lv_bar_get_value(obj)
}

// #define LV_COLOR_MAKE(r8, g8, b8) LV_CONCAT(LV_COLOR_MAKE, LV_COLOR_DEPTH)(r8, g8, b8)
#[inline(always)]
pub fn lv_color_make(r: u8, g: u8, b: u8) -> lv_color16_t {
    lv_color16_t {
        full: (((b as u16 & 0xF8) << 8) |    // R: top 5 bits
                ((r as u16 & 0xFC) << 3) |    // G: top 6 bits
                ((g as u16) >> 3)),           // B: top 5 bits
    }
}
