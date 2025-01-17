#![no_main]
#![no_std]

//! Blinks LED1 every second and lights up LED 2 when button 2 is pressed.
//! Prints messages over RTT

use core::cell::RefCell;
use core::ops::DerefMut;
use cortex_m::interrupt::{free, Mutex};
use embedded_hal::timer::CountDown;

use cortex_m_rt::entry;

use panic_rtt_target as _;

use rtt_target::{rprintln, rtt_init_print};

use bsp::hal;
use hal::pac;
use pitaya_go_bsp as bsp;

use pac::interrupt;

/// A struct used to hold the resources we need
struct Blinky {
    timer0: hal::Timer<pac::TIMER0, hal::timer::Periodic>,
    timer1: hal::Timer<pac::TIMER1, hal::timer::Periodic>,
    led_r: bsp::Led,
    led_g: bsp::Led,
    button: bsp::Button,
    led_r_on: bool,
}

/// Program resources
static BLINKY: Mutex<RefCell<Option<Blinky>>> = Mutex::new(RefCell::new(None));

#[entry]
fn main() -> ! {
    // Enable RTT printing
    rtt_init_print!();
    // Print a message over RTT
    rprintln!("Initialize");
    rprintln!("Press button to light up green LED");
    // Configure the board perephials
    if let Some(nrf52) = bsp::Board::take() {
        // Configure the timer TIMER0 to fire every second
        let mut timer0 = hal::Timer::new(nrf52.TIMER0);
        timer0.enable_interrupt();
        let mut timer0 = timer0.into_periodic();
        timer0.start(1_000_000u32);
        // Configure the timer TIMER1 to fire 50 milliseconds
        let mut timer1 = hal::Timer::new(nrf52.TIMER1);
        timer1.enable_interrupt();
        let mut timer1 = timer1.into_periodic();
        timer1.start(50_000u32);
        // Get red LED and turn it off
        let mut led_r = nrf52.leds.led_r;
        led_r.disable();
        // Get green LED and turn it off
        let mut led_g = nrf52.leds.led_g;
        led_g.disable();
        // Clear timer events
        let _ = timer0.wait();
        let _ = timer1.wait();
        // Enable interrupts
        unsafe {
            pac::NVIC::unmask(pac::Interrupt::TIMER0);
            pac::NVIC::unmask(pac::Interrupt::TIMER1);
        }
        // Create Blinky struct
        let blinky = Blinky {
            timer0,
            timer1,
            led_r,
            led_g,
            button: nrf52.buttons.button_1,
            led_r_on: false,
        };
        // Store the Blinky struct in the static variable
        free(|cs| {
            BLINKY.borrow(cs).replace(Some(blinky));
        });
    }
    // Loop endlessly
    loop {}
}

#[interrupt]
fn TIMER0() {
    // Lock the Mutex to borrow the resources
    free(|cs| {
        if let Some(ref mut blinky) = BLINKY.borrow(cs).borrow_mut().deref_mut() {
            // Wait and reset the timer event
            let _ = blinky.timer0.wait();
            // Check the state variable
            if blinky.led_r_on {
                // Enable LED 1
                blinky.led_r.enable();
            } else {
                // Disable LED 1
                blinky.led_r.disable();
            }
            // Update the state variable
            blinky.led_r_on = !blinky.led_r_on;
        }
    });
    // Print a message over RTT
    rprintln!("TIMER0");
}

#[interrupt]
fn TIMER1() {
    // Lock the Mutex to borrow the resources
    free(|cs| {
        if let Some(ref mut blinky) = BLINKY.borrow(cs).borrow_mut().deref_mut() {
            // Wait and reset the timer event
            let _ = blinky.timer1.wait();
            // Check if the button is pressed
            if blinky.button.is_pressed() {
                // Enable LED 2
                blinky.led_g.enable();
            } else {
                // Disable LED 2
                blinky.led_g.disable();
            }
        }
    });
}
