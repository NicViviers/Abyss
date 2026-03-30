/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include <stdio.h>
#include "pico/stdlib.h"
#include "../lv_port/lv_port_disp.h"
#include "../lv_port/lv_port_indev.h"
#include "demos/lv_demos.h"
#include "bsp_i2c.h"

#include "hardware/pll.h"
#include "hardware/clocks.h"
#include "hardware/structs/pll.h"
#include "hardware/structs/clocks.h"

#include "bsp_battery.h"

#define LVGL_TICK_PERIOD_MS 1

#define DISP_HOR_RES 466
#define DISP_VER_RES 466

lv_obj_t *label_adc_raw;
lv_obj_t *label_voltage;

void lvgl_battery_ui_init(void);

void set_cpu_clock(uint32_t freq_Mhz)
{
    set_sys_clock_hz(freq_Mhz * MHZ, true);
    clock_configure(
        clk_peri,
        0,
        CLOCKS_CLK_PERI_CTRL_AUXSRC_VALUE_CLKSRC_PLL_SYS,
        freq_Mhz * MHZ,
        freq_Mhz * MHZ);
}


static bool repeating_lvgl_timer_cb(struct repeating_timer *t)
{
    lv_tick_inc(LVGL_TICK_PERIOD_MS);
    return true;
}


int main()
{
    stdio_init_all();
    set_cpu_clock(250);
    bsp_i2c_init();
    bsp_battery_init();


    lv_init();
    lv_port_disp_init(DISP_HOR_RES, DISP_VER_RES, 0, false);
    lv_port_indev_init(DISP_HOR_RES, DISP_VER_RES, 0);
    static struct repeating_timer lvgl_timer;
    add_repeating_timer_ms(LVGL_TICK_PERIOD_MS, repeating_lvgl_timer_cb, NULL, &lvgl_timer);

    lvgl_battery_ui_init();
    // lv_demo_widgets();
    // lv_demo_music();
    while (true)
    {
        lv_timer_handler();
        sleep_ms(LVGL_TICK_PERIOD_MS);
    }
}


static void battery_timer_callback(lv_timer_t *timer)
{
    char str_buffer[20];
    float voltage;
    uint16_t adc_raw;
    bsp_battery_read(&voltage, &adc_raw);
    lv_label_set_text_fmt(label_adc_raw, "%d", adc_raw);
    sprintf(str_buffer, "%.1f", voltage);
    lv_label_set_text(label_voltage, str_buffer);
}

void lvgl_battery_ui_init(void)
{
    lv_obj_t *list = lv_list_create(lv_scr_act());
    lv_obj_set_size(list, lv_pct(70), lv_pct(70));
    lv_obj_center(list);

    lv_obj_t *list_item = lv_list_add_btn(list, NULL, "adc_raw");
    label_adc_raw = lv_label_create(list_item);
    lv_label_set_text(label_adc_raw, "0");

    list_item = lv_list_add_btn(list, NULL, "voltage");
    label_voltage = lv_label_create(list_item);
    lv_label_set_text(label_voltage, "0.0");

    lv_timer_create(battery_timer_callback, 1000, NULL);
}

