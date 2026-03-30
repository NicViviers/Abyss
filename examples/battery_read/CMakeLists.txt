if (TARGET tinyusb_device)
    add_executable(battery_read
            battery_read.c
            )

    # pull in common dependencies
    target_link_libraries(battery_read 
        pico_stdlib
        bsp)

    # enable usb output, disable uart output
    pico_enable_stdio_usb(battery_read 1)
    pico_enable_stdio_uart(battery_read 0)

    # create map/bin/hex/uf2 file etc.
    pico_add_extra_outputs(battery_read)

    # add url via pico_set_program_url
    # example_auto_set_url(battery_read)
elseif(PICO_ON_DEVICE)
    message("Skipping battery_read because TinyUSB submodule is not initialized in the SDK")
endif()
