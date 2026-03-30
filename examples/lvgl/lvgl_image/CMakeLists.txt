if (TARGET tinyusb_device)
    
    
    add_executable(lvgl_image
            lvgl_image.c
            ../lv_port/lv_port_disp.c
            ../lv_port/lv_port_indev.c
            ../lv_port/lv_port_fs.c
            )
    
    # pull in common dependencies
    target_link_libraries(lvgl_image 
        pico_stdlib 
        lvgl
        lvgl::demos
        bsp
        no-OS-FatFS-SD-SDIO-SPI-RPi-Pico)

    # enable usb output, disable uart output
    pico_enable_stdio_usb(lvgl_image 1)
    pico_enable_stdio_uart(lvgl_image 0)

    # create map/bin/hex/uf2 file etc.
    pico_add_extra_outputs(lvgl_image)

    # add url via pico_set_program_url
    # example_auto_set_url(lvgl_image)
elseif(PICO_ON_DEVICE)
    message("Skipping lvgl_image because TinyUSB submodule is not initialized in the SDK")
endif()
