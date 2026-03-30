/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include <stdio.h>
#include "pico/stdlib.h"
#include "bsp_battery.h"

int main()
{
    float voltage;
    uint16_t adc_raw;
    stdio_init_all();
    bsp_battery_init();

    while (true)
    {
        bsp_battery_read(&voltage, &adc_raw);
        printf("voltage:%.2f adc_value:%d\n", voltage, adc_raw);
        sleep_ms(100);
    }
}
