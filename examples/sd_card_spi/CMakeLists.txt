add_executable(sd_card_spi
    sd_card_spi.c
    )

pico_enable_stdio_usb(sd_card_spi 1)
pico_enable_stdio_uart(sd_card_spi 0)

# pull in common dependencies
target_link_libraries(sd_card_spi 
    pico_stdlib
    bsp
    no-OS-FatFS-SD-SDIO-SPI-RPi-Pico)

# create map/bin/hex/uf2 file etc.
pico_add_extra_outputs(sd_card_spi)