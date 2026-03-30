if (TARGET tinyusb_device)
    
    
    add_executable(lvgl_qmi8658
            lvgl_qmi8658.c
            ../lv_port/lv_port_disp.c
            ../lv_port/lv_port_indev.c
            )
    
    # pull in common dependencies
    target_link_libraries(lvgl_qmi8658 
        pico_stdlib 
        lvgl
        lvgl::demos
        bsp)

    # enable usb output, disable uart output
    pico_enable_stdio_usb(lvgl_qmi8658 1)
    pico_enable_stdio_uart(lvgl_qmi8658 0)

    # create map/bin/hex/uf2 file etc.
    pico_add_extra_outputs(lvgl_qmi8658)

    # add url via pico_set_program_url
    # example_auto_set_url(lvgl_qmi8658)
elseif(PICO_ON_DEVICE)
    message("Skipping lvgl_qmi8658 because TinyUSB submodule is not initialized in the SDK")
endif()
