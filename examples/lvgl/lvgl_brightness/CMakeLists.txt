if (TARGET tinyusb_device)
    
    
    add_executable(lvgl_brightness
            lvgl_brightness.c
            ../lv_port/lv_port_disp.c
            ../lv_port/lv_port_indev.c
            )
    
    # pull in common dependencies
    target_link_libraries(lvgl_brightness 
        pico_stdlib 
        lvgl
        lvgl::demos
        bsp)

    # enable usb output, disable uart output
    pico_enable_stdio_usb(lvgl_brightness 1)
    pico_enable_stdio_uart(lvgl_brightness 0)

    # create map/bin/hex/uf2 file etc.
    pico_add_extra_outputs(lvgl_brightness)

    # add url via pico_set_program_url
    # example_auto_set_url(lvgl_brightness)
elseif(PICO_ON_DEVICE)
    message("Skipping lvgl_brightness because TinyUSB submodule is not initialized in the SDK")
endif()
