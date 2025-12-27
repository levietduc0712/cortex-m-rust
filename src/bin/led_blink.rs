#![no_std]
#![no_main]

use cortex_m_rt::entry;
use cortex_m::asm::delay;
use stm32f4xx_hal::{pac, prelude::*};

// Logging + panic
use defmt::println;
use defmt_rtt as _;
use panic_probe as _;

#[entry]
fn main() -> ! {
    let dp = pac::Peripherals::take().unwrap();

    // Configure clocks
    let rcc = dp.RCC.constrain();
    let _clocks = rcc.cfgr.freeze();

    // GPIO
    let gpiod = dp.GPIOD.split();

    // LEDs (Discovery board)
    let mut green  = gpiod.pd12.into_push_pull_output();
    let mut orange = gpiod.pd13.into_push_pull_output();
    let mut red    = gpiod.pd14.into_push_pull_output();
    let mut blue   = gpiod.pd15.into_push_pull_output();

    let mut current = 0u8;

    loop {
        // Turn all LEDs off
        green.set_low();
        orange.set_low();
        red.set_low();
        blue.set_low();

        // Toggle selected LED
        match current {
            0 => {
                green.toggle();
                println!("Green LED toggled");
            }
            1 => {
                orange.toggle();
                println!("Orange LED toggled");
            }
            2 => {
                red.toggle();
                println!("Red LED toggled");
            }
            3 => {
                blue.toggle();
                println!("Blue LED toggled");
            }
            _ => {}
        }

        delay(8_000_000);
        current = (current + 1) % 4;
    }
}

// Avoid double panic printing
#[defmt::panic_handler]
fn panic() -> ! {
    cortex_m::asm::udf()
}
