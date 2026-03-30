#include "main_screen.h"
#include "bsp_co5300.h"
#include "bsp_battery.h"
#include "bsp_pcf85063.h"
#include "bsp_qmi8658.h"
#include "bsp_serial.h"
#include "hardware/adc.h"

#include "hardware/sync.h"
#include "hardware/clocks.h"
#include "hardware/flash.h"

#include "f_util.h"
#include "crash.h"
#include "hw_config.h"
#include "my_debug.h"
#include "my_rtc.h"
#include "sd_card.h"
#include "ff.h"
//
#include "diskio.h" /* Declarations of disk functions */

uint16_t menu_cont_count = 0;
extern bsp_display_interface_t *display_if;

lv_obj_t *ui_main_screen = NULL;

// Display time
lv_obj_t *label_time = NULL;
// Display date
lv_obj_t *label_date = NULL;
// Collect battery voltage ADC
lv_obj_t *label_battery_adc = NULL;
// Battery voltage
lv_obj_t *label_battery_voltage = NULL;
// Chip temperature
lv_obj_t *label_chip_temp = NULL;
// Chip frequency
lv_obj_t *label_chip_freq = NULL;
// RAM size
lv_obj_t *label_ram_size = NULL;
// Flash size
lv_obj_t *label_flash_size = NULL;
// SD card size
lv_obj_t *label_sd_size = NULL;

lv_obj_t *label_accel_x = NULL;
lv_obj_t *label_accel_y = NULL;
lv_obj_t *label_accel_z = NULL;

lv_obj_t *label_gyro_x = NULL;
lv_obj_t *label_gyro_y = NULL;
lv_obj_t *label_gyro_z = NULL;

lv_obj_t *label_uart = NULL;

lv_obj_t *label_brightness = NULL;

typedef enum
{
    LV_MENU_ITEM_BUILDER_VARIANT_1, //
    LV_MENU_ITEM_BUILDER_VARIANT_2  //
} lv_menu_builder_variant_t;

static lv_obj_t *create_menu_text(lv_obj_t *parent, const char *icon, const char *txt,
                                  lv_menu_builder_variant_t builder_variant)
{
    // Create a container
    lv_obj_t *obj = lv_menu_cont_create(parent);

    if (menu_cont_count)
        lv_obj_set_style_bg_color(obj, lv_palette_lighten(LV_PALETTE_GREEN, 1), LV_PART_MAIN);
    else
        lv_obj_set_style_bg_color(obj, lv_palette_lighten(LV_PALETTE_YELLOW, 1), LV_PART_MAIN);
    // Set background opacity
    lv_obj_set_style_bg_opa(obj, LV_OPA_60, LV_PART_MAIN);
    menu_cont_count = (menu_cont_count + 1) % 2;

    lv_obj_t *img = NULL;
    lv_obj_t *label = NULL;
    // Create an image
    if (icon)
    {
        img = lv_img_create(obj);
        lv_img_set_src(img, icon);
    }
    // Create a label
    if (txt)
    {
        label = lv_label_create(obj);
        // Set text
        lv_label_set_text(label, txt);
        // Set long mode for text overflow
        lv_label_set_long_mode(label, LV_LABEL_LONG_SCROLL_CIRCULAR);
        // Set horizontal grow
        lv_obj_set_flex_grow(label, LV_FLEX_FLOW_COLUMN);
    }
    // If variant 2, swap image and label
    if (builder_variant == LV_MENU_ITEM_BUILDER_VARIANT_2 && icon && txt)
    {
        lv_obj_add_flag(img, LV_OBJ_FLAG_FLEX_IN_NEW_TRACK);
        lv_obj_swap(img, label);
    }

    return obj;
}



static void create_menu_title(lv_obj_t *parent, const char *title, const lv_font_t *font)
{
    lv_obj_t *obj = lv_menu_cont_create(parent);
    // Set Flex layout
    lv_obj_set_flex_flow(obj, LV_FLEX_FLOW_COLUMN);                                               // Vertical arrangement
    lv_obj_set_flex_align(obj, LV_FLEX_ALIGN_CENTER, LV_FLEX_ALIGN_CENTER, LV_FLEX_ALIGN_CENTER); // Main axis, cross-axis, and content alignment are all centered

    // Create a label and set text
    lv_obj_t *label = lv_label_create(obj);
    lv_label_set_text(label, title);
    lv_obj_set_style_text_font(label, font, 0); // 20-point font
    // Optional: Set other label properties
    lv_obj_set_style_text_align(label, LV_TEXT_ALIGN_CENTER, 0); // Set text to center
}

static void swipe_event_cb(lv_event_t *e)
{
    lv_event_code_t code = lv_event_get_code(e);
    if (code == LV_EVENT_GESTURE)
    {
        lv_dir_t dir = lv_indev_get_gesture_dir(lv_indev_get_act());

        if (dir == LV_DIR_LEFT)
        {
            // Wait for touchscreen release
            lv_indev_wait_release(lv_indev_get_act());
            // Switch to drawing screen
            _ui_screen_change(&ui_drawing_screen, LV_SCR_LOAD_ANIM_FADE_ON, 500, 0, &drawing_screen_init);
        }
        else if (dir == LV_DIR_RIGHT)
        {
            lv_timer_resume(color_screen_change_timer);
            // Wait for touchscreen release
            lv_indev_wait_release(lv_indev_get_act());
            // Switch to color test screen
            _ui_screen_change(&ui_color_screen, LV_SCR_LOAD_ANIM_FADE_ON, 500, 0, &color_screen_init);
        }
    }
}

void slider_event_cb(lv_event_t *e)
{
    lv_event_code_t code = lv_event_get_code(e);
    if (code == LV_EVENT_VALUE_CHANGED)
    {
        // Get the current slider value
        lv_obj_t *slider = lv_event_get_target(e);
        int value = lv_slider_get_value(slider);
        display_if->set_brightness(value);
        // bsp_co5300_set_brightness(value);
        // printf("Slider value: %d\n", value);

        lv_label_set_text_fmt(label_brightness, "%d %%", value);
        // Prevent event from propagating upward
        lv_event_stop_bubbling(e);
    }
}

// Create horizontal slider
void create_horizontal_slider(lv_obj_t *parent)
{
    // Create slider
    lv_obj_t *slider = lv_slider_create(parent);

    // Set slider direction to horizontal
    lv_slider_set_range(slider, 1, 100);          // Set slider range
    lv_slider_set_value(slider, 80, LV_ANIM_OFF); // Set initial value

    // Adjust slider size and position
    lv_obj_set_size(slider, lv_pct(85), 45);    // Width 200, height 20
    lv_obj_align(slider, LV_ALIGN_CENTER, 0, 0); // Centered display

    lv_obj_set_style_pad_top(parent, 20, 0);
    lv_obj_set_style_pad_bottom(parent, 20, 0);
    // lv_obj_set_style_pad_left(parent, 50, 0);
    // lv_obj_set_style_pad_right(parent, 50, 0);
    lv_obj_clear_flag(parent, LV_OBJ_FLAG_GESTURE_BUBBLE);
    // Add event callback (optional)
    lv_obj_add_event_cb(slider, slider_event_cb, LV_EVENT_VALUE_CHANGED, NULL);
}


float read_chip_temp(void)
{
    /* 12-bit conversion, assume max value == ADC_VREF == 3.3 V */
    const float conversionFactor = 3.3f / (1 << 12);
    adc_select_input(4);
    float adc = (float)adc_read() * conversionFactor;
    float tempC = 27.0f - (adc - 0.706f) / 0.001721f;
    return tempC;
}


static void timer_qmi8658_cb(lv_timer_t *timer)
{
    qmi8658_data_t data;
    bsp_qmi8658_read_data(&data);
    lv_label_set_text_fmt(label_accel_x, "%d", data.acc_x);
    lv_label_set_text_fmt(label_accel_y, "%d", data.acc_y);
    lv_label_set_text_fmt(label_accel_z, "%d", data.acc_z);

    lv_label_set_text_fmt(label_gyro_x, "%d", data.gyr_x);
    lv_label_set_text_fmt(label_gyro_y, "%d", data.gyr_y);
    lv_label_set_text_fmt(label_gyro_z, "%d", data.gyr_z);
    // printf("Accel: %d, %d, %d   \t", data.acc_x, data.acc_y, data.acc_z);
    // printf("Gyro: %d, %d, %d\n", data.gyr_x, data.gyr_y, data.gyr_z);
}

static void timer_1000ms_cb(lv_timer_t *timer)
{
    float battery_voltage;
    uint16_t battery_adc_raw;
    char str[20];
    struct tm now_tm;
    float chip_temp;
    bsp_pcf85063_get_time(&now_tm);
    lv_label_set_text_fmt(label_time, "%02d:%02d:%02d", now_tm.tm_hour, now_tm.tm_min, now_tm.tm_sec);
    lv_label_set_text_fmt(label_date, "%04d-%02d-%02d", now_tm.tm_year + 1900, now_tm.tm_mon + 1, now_tm.tm_mday);
    bsp_battery_read(&battery_voltage, &battery_adc_raw);
    // printf("time %02d:%02d:%02d\n", now_tm.tm_hour, now_tm.tm_min, now_tm.tm_sec);
    
    chip_temp = read_chip_temp();
    sprintf(str, "%.1f C", chip_temp);
    lv_label_set_text(label_chip_temp, str);

    lv_label_set_text_fmt(label_battery_adc, "%d", battery_adc_raw);
    sprintf(str, "%.1f V", battery_voltage);
    lv_label_set_text(label_battery_voltage, str);
    // printf("battery_adc_raw: %d\n", battery_adc_raw);
    
}

static void timer_uart_cb(lv_timer_t *timer)
{
    uint8_t TEST_LEN = 4;
    const char test_buf[] = { 'T', 'E', 'S', 'T' };

    // 1. Send 4 bytes
    for (int i = 0; i < TEST_LEN; i++) {
        uart_putc_raw(UART_ID, test_buf[i]);
    }

    // 2. Receive 4 bytes
    int recv_cnt = 0;
    char recv_buf[TEST_LEN];

    absolute_time_t timeout = make_timeout_time_ms(10);

    while (recv_cnt < TEST_LEN) {
        if (uart_is_readable(UART_ID)) {
            recv_buf[recv_cnt++] = uart_getc(UART_ID);
        }

        if (absolute_time_diff_us(get_absolute_time(), timeout) <= 0) {
            lv_label_set_text(label_uart, "Not connect");
            return;
        }
    }

    // 3. check result
    if (recv_cnt == TEST_LEN) {
        for (int i = 0; i < TEST_LEN; i++) {
            if(recv_buf[i] != test_buf[i]){
                lv_label_set_text(label_uart, "Not connect");
                return;
            }
            if(i == TEST_LEN-1){
                lv_label_set_text(label_uart, "Connected");
                return;
            }
        }
    } else {
        lv_label_set_text(label_uart, "Not connect");
        return;
    }
}

uint32_t get_sd_card_size(void)
{
    const char *arg = "0";
    sd_card_t *sd_card_p = sd_get_by_drive_prefix(arg);
    FATFS *fs_p = &sd_card_p->state.fatfs;

    DWORD fre_clust, fre_sect, tot_sect;
    FRESULT fr = f_getfree(arg, &fre_clust, &fs_p);
    if (FR_OK != fr) {
        printf("f_getfree error: %s (%d)\n", FRESULT_str(fr), fr);
        return 0;
    }
    /* Get total sectors and free sectors */
    tot_sect = (fs_p->n_fatent - 2) * fs_p->csize;
    fre_sect = fre_clust * fs_p->csize;
    /* Print the free space (assuming 512 bytes/sector) */
    printf("\n%10lu KiB (%lu MiB) total drive space.\n%10lu KiB (%lu MiB) available.\n",
           tot_sect / 2, tot_sect / 2 / 1024,
           fre_sect / 2, fre_sect / 2 / 1024);
    
    return tot_sect / 2 / 1024;
}
void main_screen_init(void)
{
    uint8_t txbuf[4] = {0x9F, 0, 0, 0}; 
    uint8_t rxbuf[4] = {0}; 

    uint32_t interrupts = save_and_disable_interrupts();
    flash_do_cmd(txbuf, rxbuf, sizeof(txbuf));
    restore_interrupts(interrupts);

    uint32_t flash_size = 1 << rxbuf[3];
    uint32_t sys_clk = clock_get_hz(clk_sys);

    uint64_t sd_card_size_MB = get_sd_card_size();

    ui_main_screen = lv_obj_create(NULL);
    // Clear flags
    lv_obj_clear_flag(ui_main_screen, LV_OBJ_FLAG_SCROLLABLE); /// Flags

    lv_obj_add_event_cb(ui_main_screen, swipe_event_cb, LV_EVENT_GESTURE, NULL);
    // Create menu
    lv_obj_t *menu = lv_menu_create(ui_main_screen);
    // Get background color
    lv_color_t bg_color = lv_obj_get_style_bg_color(menu, 0);
    // Check if background color is light
    if (lv_color_brightness(bg_color) > 127)
    {
        // Set background color to dark
        lv_obj_set_style_bg_color(menu, lv_color_darken(lv_obj_get_style_bg_color(menu, 0), 10), 0);
    }
    else
    {
        // Set background color to light
        lv_obj_set_style_bg_color(menu, lv_color_darken(lv_obj_get_style_bg_color(menu, 0), 50), 0);
    }
    // Set menu size, width 100%, height 100%
    lv_obj_set_size(menu, lv_pct(100), lv_pct(100));
    // Center display
    lv_obj_center(menu);

    // Create main menu page
    lv_obj_t *menu_main_page = lv_menu_page_create(menu, NULL);
    // // Hide scrollbar
    // lv_obj_set_style_bg_opa(menu_main_page, LV_OPA_0, LV_PART_SCROLLBAR | LV_STATE_DEFAULT);
    // // Hide scrollbar when scrolling
    // lv_obj_set_style_bg_opa(menu_main_page, LV_OPA_0, LV_PART_SCROLLBAR | LV_STATE_SCROLLED);
    // Set padding (Margin)
    lv_obj_set_style_pad_top(menu_main_page, 10, 0);
    lv_obj_set_style_pad_bottom(menu_main_page, 10, 0);
    lv_obj_set_style_pad_left(menu_main_page, 50, 0);
    lv_obj_set_style_pad_right(menu_main_page, 50, 0);

    lv_obj_t *obj = NULL;
    lv_obj_t *label = NULL;
    lv_obj_t *section = NULL;

    obj = lv_menu_cont_create(menu_main_page);
    lv_obj_set_flex_flow(obj, LV_FLEX_FLOW_ROW); // Arrange child objects horizontally (can be changed to LV_FLEX_FLOW_COLUMN)
    lv_obj_set_flex_align(obj, LV_FLEX_ALIGN_CENTER, LV_FLEX_ALIGN_CENTER, LV_FLEX_ALIGN_CENTER);
    lv_obj_t *img = lv_img_create(obj);
    lv_img_set_src(img, &lv_logo_wx);

    create_menu_title(menu_main_page, "RP2350-Touch-AMOLED-1.43", &lv_font_montserrat_14);

    //------------------------------------Time and Date---------------------------------------------
    // Time and Date
    // obj = create_menu_text(menu_main_page, NULL, "Time and Date", LV_MENU_ITEM_BUILDER_VARIANT_1);
    create_menu_title(menu_main_page, "Time and Date", &lv_font_montserrat_20);

    // Create a menu section object
    section = lv_menu_section_create(menu_main_page);

    // Create a menu item
    obj = create_menu_text(section, NULL, "Time", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_time = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_time, "12:00:00");

    // Create a menu item
    obj = create_menu_text(section, NULL, "Date", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_date = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_date, "2024-12-01");
    //--------------------------------------------------------------------------------------

    //------------------------------------Chip---------------------------------------------
    // Chip
    create_menu_title(menu_main_page, "Chip", &lv_font_montserrat_20);
    // Create a menu section object
    section = lv_menu_section_create(menu_main_page);

    // Create a menu item
    obj = create_menu_text(section, NULL, "ChipType", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label = lv_label_create(obj);
    // Set value to RP2350A
    lv_label_set_text(label, "RP2350A");

    // Create a menu item
    obj = create_menu_text(section, NULL, "Temp", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_chip_temp = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_chip_temp, "--- C");

    // Create a menu item
    obj = create_menu_text(section, NULL, "Freq", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_chip_freq = lv_label_create(obj);
    // Set value
    lv_label_set_text_fmt(label_chip_freq, "%d MHz", sys_clk / 1000 / 1000);
    //--------------------------------------------------------------------------------------

    //------------------------------------Memory---------------------------------------------
    // Memory
    create_menu_title(menu_main_page, "Memory", &lv_font_montserrat_20);
    // Create a menu section object
    section = lv_menu_section_create(menu_main_page);

    // Create a menu item
    obj = create_menu_text(section, NULL, "RAM", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label = lv_label_create(obj);
    // Set value
    lv_label_set_text(label, "520 KB");

    // Create a menu item
    obj = create_menu_text(section, NULL, "Flash", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_flash_size = lv_label_create(obj);
    // Set value
    lv_label_set_text_fmt(label_flash_size, "%d MB", flash_size / 1024 / 1024);

    // Create a menu item
    obj = create_menu_text(section, NULL, "SDCard", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_sd_size = lv_label_create(obj);
    // Set value
    lv_label_set_text_fmt(label_sd_size, "%d MB", sd_card_size_MB);
    //--------------------------------------------------------------------------------------

    //------------------------------------Battery---------------------------------------------
    // Battery
    create_menu_title(menu_main_page, "Battery", &lv_font_montserrat_20);
    // Create a menu section object
    section = lv_menu_section_create(menu_main_page);

    // Create a menu item
    obj = create_menu_text(section, NULL, "ADC", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_battery_adc = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_battery_adc, "----");

    // Create a menu item
    obj = create_menu_text(section, NULL, "Voltage", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_battery_voltage = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_battery_voltage, "--- V");
    //--------------------------------------------------------------------------------------

    //------------------------------------QMI8658---------------------------------------------
    // QMI8658
    create_menu_title(menu_main_page, "QMI8658", &lv_font_montserrat_20);
    // Create a menu section object
    section = lv_menu_section_create(menu_main_page);

    // Create a menu item
    obj = create_menu_text(section, NULL, "Accel_x", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_accel_x = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_accel_x, "----");

    // Create a menu item
    obj = create_menu_text(section, NULL, "Accel_y", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_accel_y = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_accel_y, "----");

    // Create a menu item
    obj = create_menu_text(section, NULL, "Accel_z", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_accel_z = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_accel_z, "----");

    // Create a menu item
    obj = create_menu_text(section, NULL, "Gyro_x", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_gyro_x = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_gyro_x, "----");

    // Create a menu item
    obj = create_menu_text(section, NULL, "Gyro_y", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_gyro_y = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_gyro_y, "----");

    // Create a menu item
    obj = create_menu_text(section, NULL, "Gyro_z", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_gyro_z = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_gyro_z, "----");
    //--------------------------------------------------------------------------------------

    //------------------------------------UART---------------------------------------------
    // Brightness
    create_menu_title(menu_main_page, "UART", &lv_font_montserrat_20);
    // Create a menu section object
    section = lv_menu_section_create(menu_main_page);

    // Create a menu item
    obj = create_menu_text(section, NULL, "TX RX", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_uart = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_uart, "----");
    //--------------------------------------------------------------------------------------

    //------------------------------------Brightness---------------------------------------------
    // Brightness
    create_menu_title(menu_main_page, "Brightness", &lv_font_montserrat_20);
    // Create a menu section object
    section = lv_menu_section_create(menu_main_page);

    // Create a menu item
    obj = create_menu_text(section, NULL, "Brightness", LV_MENU_ITEM_BUILDER_VARIANT_1);
    label_brightness = lv_label_create(obj);
    // Set value
    lv_label_set_text(label_brightness, "80 %");

    obj = lv_menu_cont_create(section);
    lv_obj_set_flex_align(obj, LV_FLEX_ALIGN_CENTER, LV_FLEX_ALIGN_CENTER, LV_FLEX_ALIGN_CENTER);

    if (menu_cont_count)
        lv_obj_set_style_bg_color(obj, lv_palette_lighten(LV_PALETTE_GREEN, 1), LV_PART_MAIN);
    else
        lv_obj_set_style_bg_color(obj, lv_palette_lighten(LV_PALETTE_YELLOW, 1), LV_PART_MAIN);
    // Set background opacity
    lv_obj_set_style_bg_opa(obj, LV_OPA_60, LV_PART_MAIN);
    menu_cont_count = (menu_cont_count + 1) % 2;
    create_horizontal_slider(obj);
    //--------------------------------------------------------------------------------------

    //------------------------------------LOGO---------------------------------------------
    obj = lv_menu_cont_create(menu_main_page);
    lv_obj_set_flex_flow(obj, LV_FLEX_FLOW_ROW); // Arrange child objects horizontally (can be changed to LV_FLEX_FLOW_COLUMN)
    lv_obj_set_flex_align(obj, LV_FLEX_ALIGN_CENTER, LV_FLEX_ALIGN_CENTER, LV_FLEX_ALIGN_CENTER);
    img = lv_img_create(obj);
    lv_img_set_src(img, &lv_logo_wx);
    
    create_menu_title(menu_main_page, "RP2350-Touch-AMOLED-1.43", &lv_font_montserrat_14);
    //--------------------------------------------------------------------------------------

    lv_menu_set_page(menu, menu_main_page);
    lv_timer_create(timer_1000ms_cb, 1000, NULL);
    lv_timer_create(timer_qmi8658_cb, 100, NULL);
    lv_timer_create(timer_uart_cb, 100, NULL);
}