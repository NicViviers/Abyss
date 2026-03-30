if (TARGET tinyusb_device)
    
    
    add_executable(lvgl_battery
            lvgl_battery.c
            ../lv_port/lv_port_disp.c
            ../lv_port/lv_port_indev.c
            )
    
    # pull in common dependencies
    target_link_libraries(lvgl_battery 
        pico_stdlib 
        lvgl
        lvgl::demos
        bsp)

    # enable usb output, disable uart output
    pico_enable_stdio_usb(lvgl_battery 1)
    pico_enable_stdio_uart(lvgl_battery 0)

    # create map/bin/hex/uf2 file etc.
    pico_add_extra_outputs(lvgl_battery)

    # add url via pico_set_program_url
    # example_auto_set_url(lvgl_battery)
elseif(PICO_ON_DEVICE)
    message("Skipping lvgl_battery because TinyUSB submodule is not initialized in the SDK")
endif()
