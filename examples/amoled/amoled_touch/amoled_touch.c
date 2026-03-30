/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include <stdio.h>
#include "pico/stdlib.h"

#include "bsp_i2c.h"

#include "bsp_co5300.h"
#include "bsp_ft6146.h"

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
    bsp_i2c_init();
    bsp_display_interface_t *display_if;
    bsp_display_info_t display_info;
    display_info.height = OLED_WIDTH;
    display_info.width = OLED_HEIGHT;
    display_info.x_offset = 6;
    display_info.y_offset = 0;
    display_info.rotation = 0;
    display_info.brightness = 80;
    display_info.dma_flush_done_cb = flush_done;
    bsp_display_new_co5300(&display_if, &display_info);
    display_if->init();

    bsp_touch_info_t touch_info;
    touch_info.width = OLED_WIDTH;
    touch_info.height = OLED_HEIGHT;
    touch_info.rotation = 0;

    bsp_touch_interface_t *touch_if;
    bsp_touch_new_ft6146(&touch_if, &touch_info);
    touch_if->init();

    for (int i = 0; i < 3; i++)
    {
        swap_color((uint8_t *)&color[i]);
    }
    uint16_t color_arr[OLED_WIDTH * OLED_HEIGHT];

    for (int i = 0; i < OLED_WIDTH * OLED_HEIGHT; i++)
    {
        color_arr[i] = 0xffff;
    }
    bsp_touch_data_t data;
    uint16_t flush_count = 0;
    bsp_display_area_t area = {
        .x1 = 0,
        .x2 = OLED_WIDTH - 1,
        .y1 = 0,
        .y2 = OLED_HEIGHT - 1,
    };
    display_if->flush_dma(&area, color_arr);
    while (true)
    {
        touch_if->read();

        if (touch_if->get_data(&data))
        {
            for (int i = 0; i < data.points; i++)
            {
                printf("point: %d, x:%d, y:%d\r\n", i, data.coords[i].x, data.coords[i].y);
                if (data.coords[i].x > OLED_WIDTH - 8)
                    data.coords[i].x = OLED_WIDTH - 8;
                if (data.coords[i].y > OLED_HEIGHT - 8)
                    data.coords[i].y = OLED_HEIGHT - 8;

                for (int w = 0; w < 8; w++)
                {
                    for (int h = 0; h < 8; h++)
                    {
                        *(color_arr + OLED_WIDTH * (data.coords[i].y + h) + data.coords[0].x + w) = color[i];
                    }
                }
            }
            while (!flush_done_flag)
                sleep_us(100);
            flush_done_flag = false;
            display_if->flush_dma(&area, color_arr);
        }
        sleep_ms(10);
    }
}
