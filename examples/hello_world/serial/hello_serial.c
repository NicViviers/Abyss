/**
 * Copyright (c) 2020 Raspberry Pi (Trading) Ltd.
 *
 * SPDX-License-Identifier: BSD-3-Clause
 */

#include "bsp_serial.h"

int main() {
    bsp_serial_init();
    while(1)
    {
        uart_puts(UART_ID, "Hello, world!\n");
        sleep_ms(1000);
    }
    return 0;
}
