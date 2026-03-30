/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include <stdio.h>
#include "pico/stdlib.h"
#include "../lv_port/lv_port_disp.h"
#include "../lv_port/lv_port_indev.h"
#include "../lv_port/lv_port_fs.h"
#include "demos/lv_demos.h"
#include "bsp_i2c.h"

#include "hardware/pll.h"
#include "hardware/clocks.h"
#include "hardware/structs/pll.h"
#include "hardware/structs/clocks.h"
#define LVGL_TICK_PERIOD_MS 1

#define DISP_HOR_RES 466
#define DISP_VER_RES 466

#define MAX_FILENAME_LENGTH 100
#define MAX_FILE_COUNT 100

char bin_filenames[MAX_FILE_COUNT][MAX_FILENAME_LENGTH];
int bin_file_count = 0;
lv_obj_t *img = NULL;


static void img_gesture_event_cb(lv_event_t *e);
void print_bin_filenames(void);
void read_bin_files(const char *dir_path);
static void img_callback(lv_timer_t *timer);


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
    lv_obj_t *obj;
    char str_buf[300];
    stdio_init_all();
    // set_cpu_clock(250);
    bsp_i2c_init();
    
    lv_init();
    lv_port_fs_init();
    lv_port_disp_init(DISP_HOR_RES, DISP_VER_RES, 0, false);
    lv_port_indev_init(DISP_HOR_RES, DISP_VER_RES, 0);
    static struct repeating_timer lvgl_timer;
    add_repeating_timer_ms(LVGL_TICK_PERIOD_MS, repeating_lvgl_timer_cb, NULL, &lvgl_timer);



    read_bin_files("0:images");
    print_bin_filenames();
    img = lv_img_create(lv_scr_act());
    lv_obj_set_size(img, DISP_HOR_RES, DISP_VER_RES);
    sprintf(str_buf, "0:images/%s", bin_filenames[0]);
    lv_img_set_src(img, str_buf);

    lv_obj_clear_flag(img, LV_OBJ_FLAG_SCROLLABLE); 
    lv_obj_add_flag(img, LV_OBJ_FLAG_GESTURE_BUBBLE);
    lv_obj_add_event_cb(lv_scr_act(), img_gesture_event_cb, LV_EVENT_GESTURE, NULL);
    
    // lv_timer_create(img_callback, 2000, NULL);


    while (true)
    {
        lv_timer_handler();
        sleep_ms(LVGL_TICK_PERIOD_MS);
    }
}



int is_bin_file(const char *filename)
{
    const char *ext = strrchr(filename, '.');
    return (ext != NULL && strcmp(ext, ".bin") == 0);
}

void read_bin_files(const char *dir_path)
{
    lv_fs_dir_t dir;
    lv_fs_res_t res = lv_fs_dir_open(&dir, dir_path);
    if (res != LV_FS_RES_OK)
    {
        return;
    }
    char filename[MAX_FILENAME_LENGTH];
    while (lv_fs_dir_read(&dir, filename) == LV_FS_RES_OK && filename[0] != '\0')
    {
        if (is_bin_file(filename))
        {
            if (bin_file_count < MAX_FILE_COUNT)
            {
                strncpy(bin_filenames[bin_file_count], filename, MAX_FILENAME_LENGTH - 1);
                bin_filenames[bin_file_count][MAX_FILENAME_LENGTH - 1] = '\0';
                bin_file_count++;
            }
            else
            {
                break;
            }
        }
    }
    lv_fs_dir_close(&dir);
}

void print_bin_filenames(void)
{
    for (int i = 0; i < bin_file_count; i++)
    {
        printf("Found .bin file: %s \r\n", bin_filenames[i]);
    }
}


static void img_callback(lv_timer_t *timer)
{
    char str_buf[300];
    static uint16_t img_index = 0;
    sprintf(str_buf, "0:images/%s", bin_filenames[img_index]);
    lv_img_set_src(img, str_buf);
    if (++img_index >= bin_file_count)
    {
        img_index = 0;
    }
}


static void img_gesture_event_cb(lv_event_t *e)
{
    char str_buf[300];
    static int img_index = 0;

    lv_event_code_t code = lv_event_get_code(e);
    
    if (code == LV_EVENT_GESTURE)
    {
        // printf("img_gesture_event_cb\r\n");
        lv_dir_t dir = lv_indev_get_gesture_dir(lv_indev_get_act());

        if (dir == LV_DIR_LEFT)
        {
            if (++img_index >= bin_file_count)
            {
                img_index = 0;
            }
            lv_indev_wait_release(lv_indev_get_act());
        }
        else if (dir == LV_DIR_RIGHT)
        {
            if (--img_index < 0)
            {
                img_index = bin_file_count - 1;
            }
            lv_indev_wait_release(lv_indev_get_act());
        }
        sprintf(str_buf, "0:images/%s", bin_filenames[img_index]);
        lv_img_set_src(img, str_buf);
    }
}
