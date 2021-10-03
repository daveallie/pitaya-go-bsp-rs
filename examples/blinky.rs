#![no_main]
#![no_std]

use cortex_m_rt::entry;
use nb::block;

use panic_rtt_target as _;
use rtt_target::{rprintln, rtt_init_print};

use pitaya_go_bsp::{
    hal::{
        prelude::*,
        timer::{self, Timer},
    },
    Board,
};

#[entry]
fn main() -> ! {
    // Enable RTT printing
    rtt_init_print!();
    // Print a message over RTT
    rprintln!("Initialize");

    let mut nrf52 = Board::take().unwrap();
    let mut timer = Timer::new(nrf52.TIMER0);

    nrf52.leds.led_r.disable();
    nrf52.leds.led_g.disable();
    nrf52.leds.led_b.disable();

    // cycle through LED colors
    loop {
        rprintln!("Red");
        nrf52.leds.led_r.enable();
        delay(&mut timer, 500_000);
        nrf52.leds.led_r.disable();

        rprintln!("Green");
        nrf52.leds.led_g.enable();
        delay(&mut timer, 500_000);
        nrf52.leds.led_g.disable();

        rprintln!("Blue");
        nrf52.leds.led_b.enable();
        delay(&mut timer, 500_000);
        nrf52.leds.led_b.disable();

        rprintln!("Yellow");
        nrf52.leds.led_r.enable();
        nrf52.leds.led_g.enable();
        delay(&mut timer, 500_000);
        nrf52.leds.led_r.disable();
        nrf52.leds.led_g.disable();

        rprintln!("Cyan");
        nrf52.leds.led_g.enable();
        nrf52.leds.led_b.enable();
        delay(&mut timer, 500_000);
        nrf52.leds.led_g.disable();
        nrf52.leds.led_b.disable();

        rprintln!("Magenta");
        nrf52.leds.led_r.enable();
        nrf52.leds.led_b.enable();
        delay(&mut timer, 500_000);
        nrf52.leds.led_r.disable();
        nrf52.leds.led_b.disable();

        rprintln!("White");
        nrf52.leds.led_r.enable();
        nrf52.leds.led_g.enable();
        nrf52.leds.led_b.enable();
        delay(&mut timer, 500_000);
        nrf52.leds.led_r.disable();
        nrf52.leds.led_g.disable();
        nrf52.leds.led_b.disable();
    }
}

fn delay<T>(timer: &mut Timer<T>, cycles: u32)
where
    T: timer::Instance,
{
    timer.start(cycles);
    let _ = block!(timer.wait());
}
