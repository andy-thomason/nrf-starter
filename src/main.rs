#![no_std]
#![no_main]

use core::panic::PanicInfo;

use nrf52840_hal as _;
use rtt_target::{rprintln, rtt_init_print}; // memory layout


#[cortex_m_rt::entry]
fn main() -> ! {
    rtt_init_print!();
    rprintln!("hello world!");
    loop {
        cortex_m::asm::bkpt();
    }
}

#[panic_handler]
fn panic_handler(_: &PanicInfo) -> ! {
    loop {
        cortex_m::asm::bkpt();
    }
}
