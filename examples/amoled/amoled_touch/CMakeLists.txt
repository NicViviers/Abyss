if (TARGET tinyusb_device)
    add_executable(amoled_touch
            amoled_touch.c
            )

    # pull in common dependencies
    target_link_libraries(amoled_touch 
        pico_stdlib
        bsp
    )

    # enable usb output, disable uart output
    pico_enable_stdio_usb(amoled_touch 1)
    pico_enable_stdio_uart(amoled_touch 0)

    # create map/bin/hex/uf2 file etc.
    pico_add_extra_outputs(amoled_touch)

    # add url via pico_set_program_url
    # example_auto_set_url(amoled_touch)
elseif(PICO_ON_DEVICE)
    message("Skipping amoled_touch because TinyUSB submodule is not initialized in the SDK")
endif()
