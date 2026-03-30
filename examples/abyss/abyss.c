/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include <stdio.h>
#include "pico/stdlib.h"
#include "lv_port_disp.h"
#include "lv_port_indev.h"
#include "demos/lv_demos.h"
#include "bsp_i2c.h"
#include "bsp_co5300.h"

#include "hardware/pll.h"
#include "hardware/clocks.h"
#include "hardware/structs/pll.h"
#include "hardware/structs/clocks.h"

extern int entry();

// Add a global variable
bsp_display_interface_t *g_display_ptr = NULL;

int main()
{
    // stdio_init_all();
    // set_cpu_clock(240);
    // bsp_i2c_init();
    entry();
    
    // lv_init();
    // lv_port_disp_init(DISP_HOR_RES, DISP_VER_RES, 0, false);
    // lv_port_indev_init(DISP_HOR_RES, DISP_VER_RES, 0);
    // static struct repeating_timer lvgl_timer;
    // add_repeating_timer_ms(LVGL_TICK_PERIOD_MS, repeating_lvgl_timer_cb, NULL, &lvgl_timer);
    // lv_demo_widgets();
    // // lv_demo_music();
    // uint16_t lvgl_ms = LVGL_TICK_PERIOD_MS;
    // while (true)
    // {
    //     lvgl_ms = lv_timer_handler();
    //     if (lvgl_ms > 500)
    //         lvgl_ms = 500;
    //     if(lvgl_ms < LVGL_TICK_PERIOD_MS)
    //         lvgl_ms = LVGL_TICK_PERIOD_MS;
    //     sleep_ms(lvgl_ms);
    // }
}
