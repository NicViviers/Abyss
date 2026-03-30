if(TARGET tinyusb_device OR EXISTS "${PICO_SDK_PATH}/lib/tinyusb")
    add_executable(abyss
            abyss.c
            ../lvgl/lv_port/lv_port_disp.c
            ../lvgl/lv_port/lv_port_indev.c
            )
    
    target_include_directories(abyss PRIVATE 
        ${CMAKE_CURRENT_SOURCE_DIR}
        ${CMAKE_CURRENT_SOURCE_DIR}/../lvgl/lv_port
    )

    # Path to your Rust library
    set(RUST_LIB "${CMAKE_SOURCE_DIR}/kernel/target/thumbv8m.main-none-eabi/release/libkernel.a")

    target_link_libraries(abyss 
        pico_stdlib 
        lvgl
        lvgl::demos
        bsp
        ${RUST_LIB}
    )

    pico_enable_stdio_usb(abyss 1)
    pico_enable_stdio_uart(abyss 0)
    pico_add_extra_outputs(abyss)

elseif(PICO_ON_DEVICE)
    message("Skipping abyss because TinyUSB submodule is not initialized")
endif()