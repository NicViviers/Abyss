#ifndef WRAPPER_H
#define WRAPPER_H

#define PICO_RP2350 1
#define PICO_ON_DEVICE 1

#include "pico/stdlib.h"
#include "pico/time.h"
#include "hardware/clocks.h"
#include "lvgl.h"

#include "bsp_display.h"
#include "bsp_qmi8658.h"
#include "bsp/bsp_touch.h"
#include "bsp/bsp_co5300.h"
#include "bsp_i2c.h"
#include "pio_qspi.h"
#include "lv_port_disp.h"
#include "lv_port_indev.h"
#include "lv_indev.h"
#include "lv_indev_scroll.h"
#include "lv_disp.h"
#include "lv_slider.h"
#include "lv_anim.h"

typedef bsp_display_interface_t* force_bindgen_to_see_this;
extern bsp_display_interface_t *display_if;

#endif