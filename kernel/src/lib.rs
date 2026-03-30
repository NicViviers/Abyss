#![no_std]
#![feature(never_type)]
#![allow(non_camel_case_types, non_upper_case_globals, non_snake_case)]
#![allow(unsafe_op_in_unsafe_fn)]

use core::panic::PanicInfo;
use rp235x_hal as hal;
use hal::multicore::{Multicore, Stack};
use hal::pac::UART0;
use hal::uart::{Enabled, UartPeripheral, DataBits, StopBits, UartConfig};
use hal::gpio::{FunctionUart, Pin, PullDown};
use hal::gpio::bank0::{Gpio0, Gpio1};
use hal::fugit::RateExtU32;
use hal::Clock;
use embedded_hal::delay::DelayNs; 
use spin::Mutex;

mod sensors;
mod config;
mod ffi;
mod deco;
mod display;
use display::{driver::Display, pages::*};

// --- Global Console Type & Static ---
type CONSOLE_T = UartPeripheral<Enabled, UART0, (Pin<Gpio0, FunctionUart, PullDown>, Pin<Gpio1, FunctionUart, PullDown>)>;
pub static CONSOLE: Mutex<Option<CONSOLE_T>> = Mutex::new(None);

// --- Core 1 Stack ---
static mut CORE1_STACK: Stack<4096> = Stack::new();

// --- LVGL Timer ---
static mut LVGL_TIMER: ffi::repeating_timer = unsafe { core::mem::zeroed() };

// --- Central Logging Logic ---
pub fn _print(args: core::fmt::Arguments) {
    let mut lock = CONSOLE.lock();
    if let Some(uart) = lock.as_mut() {
        let _ = <CONSOLE_T as core::fmt::Write>::write_fmt(uart, args);
        
        // HARDWARE DRAIN: Wait for the UART Flag Register (UARTFR) Busy bit (bit 3) to clear.
        // This ensures the shift register is empty before the CPU continues.
        unsafe {
            let uartfr = 0x4003_0018 as *const u32;
            while (core::ptr::read_volatile(uartfr) & (1 << 3)) != 0 {
                core::hint::spin_loop();
            }
        }
    }
}

// --- Macros ---
#[macro_export]
macro_rules! info {
    ($($arg:tt)*) => ($crate::_print(format_args!("[info] {}\r\n", format_args!($($arg)*))));
}

#[macro_export]
macro_rules! warn {
    ($($arg:tt)*) => ($crate::_print(format_args!("[warn] {}\r\n", format_args!($($arg)*))));
}

#[macro_export]
macro_rules! error {
    ($($arg:tt)*) => ($crate::_print(format_args!("[error] {}\r\n", format_args!($($arg)*))));
}

// --- Panic Handler ---
#[panic_handler]
fn panic_handler(info: &PanicInfo) -> ! {
    error!("Kernel panic; dumping trace then halting...");
    _print(format_args!("{}", info));

    loop { core::hint::spin_loop(); }
}

// --- Entry Point ---
#[unsafe(no_mangle)]
pub extern "C" fn entry() -> i32 {
    let mut pac = unsafe { hal::pac::Peripherals::steal() };
    let mut watchdog = hal::watchdog::Watchdog::new(pac.WATCHDOG);

    // Initialize Clocks to 150MHz (Production Speed)
    let clocks = hal::clocks::init_clocks_and_plls(
        12_000_000, // 12MHz XOSC
        pac.XOSC,
        pac.CLOCKS,
        pac.PLL_SYS,
        pac.PLL_USB,
        &mut pac.RESETS,
        &mut watchdog,
    ).ok().expect("Clock initialization failed");

    let mut sio = hal::Sio::new(pac.SIO);
    let pins = hal::gpio::Pins::new(pac.IO_BANK0, pac.PADS_BANK0, sio.gpio_bank0, &mut pac.RESETS);

    // Initialize UART0 on GP0 (TX) and GP1 (RX)
    let uart_tx = pins.gpio0.into_function::<hal::gpio::FunctionUart>();
    let uart_rx = pins.gpio1.into_function::<hal::gpio::FunctionUart>();

    let mut uart = hal::uart::UartPeripheral::new(pac.UART0, (uart_tx, uart_rx), &mut pac.RESETS)
        .enable(
            UartConfig::new(115_200.Hz(), DataBits::Eight, None, StopBits::One),
            clocks.peripheral_clock.freq(),
        ).unwrap();

    // Ensure all interrupts are disabled to prevent interference with synchronous logging
    uart.disable_rx_interrupt();
    uart.disable_tx_interrupt();

    // Hand off the UART to the global static
    {
        let mut lock = CONSOLE.lock();
        *lock = Some(uart);
    }

    info!("Abyss Kernel Online");
    info!("Clock speed: {} MHz", clocks.system_clock.freq().to_MHz());

    let mut timer = hal::Timer::new_timer0(pac.TIMER0, &mut pac.RESETS, &clocks);
    let mut timer_2 = hal::Timer::new_timer1(pac.TIMER1, &mut pac.RESETS, &clocks);

    let mut display = Display::new(timer);
    let mut dive_ui = dive(display.screen);

    unsafe {
        let mut mc = Multicore::new(&mut pac.PSM, &mut pac.PPB, &mut sio.fifo);
        let cores = mc.cores();
        let core1 = &mut cores[1];

        #[allow(static_mut_refs)]
        let _ = core1.spawn(CORE1_STACK.take().unwrap(), deco::dive_loop);

        let _sio = &*rp235x_pac::SIO::ptr();
        _sio.fifo_wr().write(|w| w.bits(core::ptr::addr_of_mut!(timer_2) as u32));
    }

    display.main_loop(|| {
        // TODO: Read button input here

        // Refresh data on the UI from global atomics
        dive_ui.sync();
    });

    // 4. Main Execution Loop
    loop {
        info!("System Heartbeat");
        timer.delay_ms(5_000);
    }
}