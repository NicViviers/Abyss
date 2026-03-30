/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include <stdio.h>
#include "pico/stdlib.h"
#include "bsp_co5300.h"

#define OLED_WIDTH 466
#define OLED_HEIGHT 466

uint16_t color_index = 0;
uint16_t color[] = {0xf800, 0x07e0, 0x001f};
bool flush_done_flag = true;

void swap_color(uint8_t *color)
{
    uint8_t swap = *color;
    *color = *(color + 1);
    *(color + 1) = swap;
}

void flush_done(void)
{
    flush_done_flag = true;
}

int main()
{
    stdio_init_all();
    uint8_t brightness = 80;
    uint16_t color_arr[OLED_WIDTH * OLED_HEIGHT];
    bsp_display_interface_t *display_if;
    bsp_display_info_t display_info;
    display_info.height = OLED_WIDTH;
    display_info.width = OLED_HEIGHT;
    display_info.x_offset = 6;
    display_info.y_offset = 0;
    display_info.rotation = 0;
    display_info.brightness = brightness;
    display_info.dma_flush_done_cb = flush_done;
    bsp_display_new_co5300(&display_if, &display_info);
    display_if->init();
    bsp_display_area_t area = {
        .x1 = 0,
        .x2 = OLED_WIDTH - 1,
        .y1 = 0,
        .y2 = OLED_HEIGHT - 1,
    };
    while (true)
    {
        printf("Hello, world!\n");

        while (!flush_done_flag)
            sleep_us(100);
        flush_done_flag = false;

        for (int i = 0; i < OLED_WIDTH * OLED_HEIGHT; i++)
        {
            color_arr[i] = color[color_index];
        }
        display_if->flush_dma(&area, color_arr);
        if (++color_index > 2)
        {
            color_index = 0;
        }
        display_if->set_brightness(brightness);
        brightness += 5;
        if (brightness >= 100)
        {
            brightness = 0;
        }
        sleep_ms(1000);
    }
}
