if (TARGET tinyusb_device)
    add_executable(amoled_flush_rgb
            amoled_flush_rgb.c
            )

    # pull in common dependencies
    target_link_libraries(amoled_flush_rgb 
        pico_stdlib
        bsp
    )

    # enable usb output, disable uart output
    pico_enable_stdio_usb(amoled_flush_rgb 1)
    pico_enable_stdio_uart(amoled_flush_rgb 0)

    # create map/bin/hex/uf2 file etc.
    pico_add_extra_outputs(amoled_flush_rgb)

    # add url via pico_set_program_url
    # example_auto_set_url(amoled_flush_rgb)
elseif(PICO_ON_DEVICE)
    message("Skipping amoled_flush_rgb because TinyUSB submodule is not initialized in the SDK")
endif()
