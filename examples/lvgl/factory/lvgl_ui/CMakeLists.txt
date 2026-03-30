# # 查找当前目录下的所有源文件
# # 并将名称保存到 DIR_Config_SRCS 变量
# aux_source_directory(. DIR_LVGL_UI_SRCS)

# aux_source_directory("./screen" DIR_LVGL_UI_SRCS)
# aux_source_directory("./image" DIR_LVGL_UI_SRCS)

# # 生成链接库
# add_library(lvgl_ui ${DIR_LVGL_UI_SRCS})


# target_link_libraries(lvgl_ui PUBLIC 
#     lvgl)
    
# # Add the standard include files to the build
# target_include_directories(lvgl_ui PRIVATE
#   ${CMAKE_CURRENT_LIST_DIR}
#   ${CMAKE_CURRENT_LIST_DIR}/.. # for our common lwipopts or any other standard includes, if required
# )


add_library(lvgl_ui INTERFACE)

aux_source_directory(${CMAKE_CURRENT_LIST_DIR}  DIR_LVGL_UI_SRCS)
aux_source_directory(${CMAKE_CURRENT_LIST_DIR}/screen DIR_LVGL_UI_SRCS)
aux_source_directory(${CMAKE_CURRENT_LIST_DIR}/image DIR_LVGL_UI_SRCS)

target_include_directories(lvgl_ui INTERFACE
  ${CMAKE_CURRENT_LIST_DIR}
  ${CMAKE_CURRENT_LIST_DIR}/screen
)

target_sources(lvgl_ui INTERFACE
  ${DIR_LVGL_UI_SRCS}
)

target_link_libraries(lvgl_ui INTERFACE 
    lvgl
    bsp
    hardware_adc
    no-OS-FatFS-SD-SDIO-SPI-RPi-Pico)
