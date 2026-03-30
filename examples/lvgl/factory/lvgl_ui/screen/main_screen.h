#ifndef __MAIN_SCREEN_H__
#define __MAIN_SCREEN_H__

#include "../lvgl_ui.h"

extern lv_obj_t *ui_main_screen;

// 显示时间
extern lv_obj_t *label_time;
// 显示日期
extern lv_obj_t *label_date;
// 采集电池电压的adc
extern lv_obj_t *label_battery_adc;
// 电池电压
extern lv_obj_t *label_battery_voltage;
// 芯片的温度
extern lv_obj_t *label_chip_temp;
// 芯片的频率
extern lv_obj_t *label_chip_freq;
// 内存大小
extern lv_obj_t *label_ram_size;
// flash大小
extern lv_obj_t *label_flash_size;
// sd 大小
extern lv_obj_t *label_sd_size;

extern lv_obj_t *label_accel_x;
extern lv_obj_t *label_accel_y;
extern lv_obj_t *label_accel_z;

extern lv_obj_t *label_gyro_x;
extern lv_obj_t *label_gyro_y;
extern lv_obj_t *label_gyro_z;

void main_screen_init(void);


#endif