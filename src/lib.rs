//! Board support crate for the Makerdiary Pitaya Go
//!
//! UARTE, SPIM and TWI should be functional,
//! but might miss some features.
#![no_std]

pub use cortex_m;
pub use embedded_hal;
pub use nrf52840_hal as hal;

/// Exports traits that are usually needed when using this crate
pub mod prelude {
    pub use nrf52840_hal::prelude::*;
}

// TODO: Maybe we want a debug module like in the DWM1001-Dev implementation.
// pub mod debug;

use nrf52840_hal::{
    gpio::{p0, p1, Disconnected, Input, Level, Output, Pin, PullUp, PushPull},
    pac::{self as nrf52, CorePeripherals, Peripherals},
    spim::{self, Frequency, Spim, MODE_0},
    uarte::{self, Baudrate as UartBaudrate, Parity as UartParity, Uarte},
};

use embedded_hal::digital::v2::{InputPin, OutputPin};

/// Provides access to all features of the Pitaya Go board
#[allow(non_snake_case)]
pub struct Board {
    /// The nRF52's pins which are not otherwise occupied on the Pitaya Go
    pub pins: Pins,

    /// The Pitaya Go UART which is wired to the virtual USB CDC port
    pub cdc: Uarte<nrf52::UARTE0>,

    /// The Pitaya Go SPI which is wired to the SPI flash
    pub flash: Spim<nrf52::SPIM2>,
    pub flash_cs: Pin<Output<PushPull>>,

    /// The LEDs on the Pitaya Go board
    pub leds: Leds,

    /// The buttons on the Pitaya Go board
    pub buttons: Buttons,

    pub nfc: NFC,

    /// Core peripheral: Cache and branch predictor maintenance operations
    pub CBP: nrf52::CBP,

    /// Core peripheral: CPUID
    pub CPUID: nrf52::CPUID,

    /// Core peripheral: Debug Control Block
    pub DCB: nrf52::DCB,

    /// Core peripheral: Data Watchpoint and Trace unit
    pub DWT: nrf52::DWT,

    /// Core peripheral: Flash Patch and Breakpoint unit
    pub FPB: nrf52::FPB,

    /// Core peripheral: Floating Point Unit
    pub FPU: nrf52::FPU,

    /// Core peripheral: Instrumentation Trace Macrocell
    pub ITM: nrf52::ITM,

    /// Core peripheral: Memory Protection Unit
    pub MPU: nrf52::MPU,

    /// Core peripheral: Nested Vector Interrupt Controller
    pub NVIC: nrf52::NVIC,

    /// Core peripheral: System Control Block
    pub SCB: nrf52::SCB,

    /// Core peripheral: SysTick Timer
    pub SYST: nrf52::SYST,

    /// Core peripheral: Trace Port Interface Unit
    pub TPIU: nrf52::TPIU,

    /// nRF52 peripheral: FICR
    pub FICR: nrf52::FICR,

    /// nRF52 peripheral: UICR
    pub UICR: nrf52::UICR,

    /// nRF52 peripheral: ACL
    pub ACL: nrf52::ACL,

    /// nRF52 peripheral: POWER
    pub POWER: nrf52::POWER,

    /// nRF52 peripheral: CLOCK
    pub CLOCK: nrf52::CLOCK,

    /// nRF52 peripheral: RADIO
    pub RADIO: nrf52::RADIO,

    /// nRF52 peripheral: UART0
    pub UART0: nrf52::UART0,

    /// nRF52 peripheral: SPIM0
    pub SPIM0: nrf52::SPIM0,

    /// nRF52 peripheral: SPIS0
    pub SPIS0: nrf52::SPIS0,

    /// nRF52 peripheral: TWIM0
    pub TWIM0: nrf52::TWIM0,

    /// nRF52 peripheral: TWIS0
    pub TWIS0: nrf52::TWIS0,

    /// nRF52 peripheral: SPI0
    pub SPI0: nrf52::SPI0,

    /// nRF52 peripheral: TWI0
    pub TWI0: nrf52::TWI0,

    /// nRF52 peripheral: SPIM1
    pub SPIM1: nrf52::SPIM1,

    /// nRF52 peripheral: SPIS1
    pub SPIS1: nrf52::SPIS1,

    /// nRF52 peripheral: TWIM1
    pub TWIM1: nrf52::TWIM1,

    /// nRF52 peripheral: TWIS1
    pub TWIS1: nrf52::TWIS1,

    /// nRF52 peripheral: SPI1
    pub SPI1: nrf52::SPI1,

    /// nRF52 peripheral: TWI1
    pub TWI1: nrf52::TWI1,

    /// nRF52 peripheral: NFCT
    pub NFCT: nrf52::NFCT,

    /// nRF52 peripheral: GPIOTE
    pub GPIOTE: nrf52::GPIOTE,

    /// nRF52 peripheral: SAADC
    pub SAADC: nrf52::SAADC,

    /// nRF52 peripheral: TIMER0
    pub TIMER0: nrf52::TIMER0,

    /// nRF52 peripheral: TIMER1
    pub TIMER1: nrf52::TIMER1,

    /// nRF52 peripheral: TIMER2
    pub TIMER2: nrf52::TIMER2,

    /// nRF52 peripheral: RTC0
    pub RTC0: nrf52::RTC0,

    /// nRF52 peripheral: TEMP
    pub TEMP: nrf52::TEMP,

    /// nRF52 peripheral: RNG
    pub RNG: nrf52::RNG,

    /// nRF52 peripheral: ECB
    pub ECB: nrf52::ECB,

    /// nRF52 peripheral: CCM
    pub CCM: nrf52::CCM,

    /// nRF52 peripheral: AAR
    pub AAR: nrf52::AAR,

    /// nRF52 peripheral: WDT
    pub WDT: nrf52::WDT,

    /// nRF52 peripheral: RTC1
    pub RTC1: nrf52::RTC1,

    /// nRF52 peripheral: QDEC
    pub QDEC: nrf52::QDEC,

    /// nRF52 peripheral: COMP
    pub COMP: nrf52::COMP,

    /// nRF52 peripheral: LPCOMP
    pub LPCOMP: nrf52::LPCOMP,

    /// nRF52 peripheral: SWI0
    pub SWI0: nrf52::SWI0,

    /// nRF52 peripheral: EGU0
    pub EGU0: nrf52::EGU0,

    /// nRF52 peripheral: SWI1
    pub SWI1: nrf52::SWI1,

    /// nRF52 peripheral: EGU1
    pub EGU1: nrf52::EGU1,

    /// nRF52 peripheral: SWI2
    pub SWI2: nrf52::SWI2,

    /// nRF52 peripheral: EGU2
    pub EGU2: nrf52::EGU2,

    /// nRF52 peripheral: SWI3
    pub SWI3: nrf52::SWI3,

    /// nRF52 peripheral: EGU3
    pub EGU3: nrf52::EGU3,

    /// nRF52 peripheral: SWI4
    pub SWI4: nrf52::SWI4,

    /// nRF52 peripheral: EGU4
    pub EGU4: nrf52::EGU4,

    /// nRF52 peripheral: SWI5
    pub SWI5: nrf52::SWI5,

    /// nRF52 peripheral: EGU5
    pub EGU5: nrf52::EGU5,

    /// nRF52 peripheral: TIMER3
    pub TIMER3: nrf52::TIMER3,

    /// nRF52 peripheral: TIMER4
    pub TIMER4: nrf52::TIMER4,

    /// nRF52 peripheral: PWM0
    pub PWM0: nrf52::PWM0,

    /// nRF52 peripheral: PDM
    pub PDM: nrf52::PDM,

    /// nRF52 peripheral: NVMC
    pub NVMC: nrf52::NVMC,

    /// nRF52 peripheral: PPI
    pub PPI: nrf52::PPI,

    /// nRF52 peripheral: MWU
    pub MWU: nrf52::MWU,

    /// nRF52 peripheral: PWM1
    pub PWM1: nrf52::PWM1,

    /// nRF52 peripheral: PWM2
    pub PWM2: nrf52::PWM2,

    /// nRF52 peripheral: RTC2
    pub RTC2: nrf52::RTC2,

    /// nRF52 peripheral: I2S
    pub I2S: nrf52::I2S,
}

impl Board {
    /// Take the peripherals safely
    ///
    /// This method will return an instance of `Board` the first time it is
    /// called. It will return only `None` on subsequent calls.
    pub fn take() -> Option<Self> {
        Some(Self::new(CorePeripherals::take()?, Peripherals::take()?))
    }

    /// Steal the peripherals
    ///
    /// This method produces an instance of `Board`, regardless of whether
    /// another instance was create previously.
    ///
    /// # Safety
    ///
    /// This method can be used to create multiple instances of `Board`. Those
    /// instances can interfere with each other, causing all kinds of unexpected
    /// behavior and circumventing safety guarantees in many ways.
    ///
    /// Always use `Board::take`, unless you really know what you're doing.
    pub unsafe fn steal() -> Self {
        Self::new(CorePeripherals::steal(), Peripherals::steal())
    }

    fn new(cp: CorePeripherals, p: Peripherals) -> Self {
        let pins0 = p0::Parts::new(p.P0);
        let pins1 = p1::Parts::new(p.P1);

        // The Pitaya Go has an 64MB SPI flash on board which can be interfaced through SPI or Quad SPI.
        // As for now, only the normal SPI mode is available, so we are using this for the interface.
        let flash_spim = Spim::new(
            p.SPIM2,
            spim::Pins {
                sck: pins1.p1_04.into_push_pull_output(Level::Low).degrade(),
                mosi: Some(pins1.p1_06.into_push_pull_output(Level::Low).degrade()),
                miso: Some(pins1.p1_01.into_floating_input().degrade()),
            },
            Frequency::K500,
            MODE_0,
            0,
        );

        let flash_cs = pins1.p1_03.into_push_pull_output(Level::High).degrade();

        // The Pitaya Go features an USB CDC port.
        // It features HWFC but does not have to use it.
        // It can transmit a flexible baudrate of up to 1Mbps.
        let cdc_uart = Uarte::new(
            p.UARTE0,
            uarte::Pins {
                txd: pins0.p0_26.into_push_pull_output(Level::High).degrade(),
                rxd: pins0.p0_27.into_floating_input().degrade(),
                cts: None, // Some(pins0.p0_07.into_floating_input().degrade()),
                rts: None, // Some(pins0.p0_05.into_push_pull_output(Level::High).degrade()),
            },
            UartParity::EXCLUDED,
            UartBaudrate::BAUD115200,
        );

        Board {
            cdc: cdc_uart,
            flash: flash_spim,
            flash_cs,

            pins: Pins {
                _RESET: pins0.p0_19,
                P13: pins0.p0_13,
                P14: pins0.p0_14,
                P15: pins0.p0_15,
                P16: pins0.p0_16,
                P17: pins0.p0_17,
                P20: pins0.p0_20,
                P21: pins0.p0_21,
                P22: pins0.p0_22,
                P23: pins0.p0_23,
                P24: pins0.p0_24,
                P25: pins0.p0_25,
                A1: pins0.p0_03,
                A2: pins0.p0_04,
                A3: pins0.p0_05,
                A4: pins0.p0_28,
                A5: pins0.p0_29,
                A6: pins0.p0_30,
                A7: pins0.p0_31,
            },

            leds: Leds {
                led_r: Led::new(pins1.p1_10.degrade()),
                led_g: Led::new(pins1.p1_11.degrade()),
                led_b: Led::new(pins1.p1_12.degrade()),
            },

            buttons: Buttons {
                button_1: Button::new(pins1.p1_00.degrade()),
            },

            nfc: NFC {
                nfc_1: pins0.p0_09,
                nfc_2: pins0.p0_10,
            },

            // Core peripherals
            CBP: cp.CBP,
            CPUID: cp.CPUID,
            DCB: cp.DCB,
            DWT: cp.DWT,
            FPB: cp.FPB,
            FPU: cp.FPU,
            ITM: cp.ITM,
            MPU: cp.MPU,
            NVIC: cp.NVIC,
            SCB: cp.SCB,
            SYST: cp.SYST,
            TPIU: cp.TPIU,

            // nRF52 peripherals
            FICR: p.FICR,
            UICR: p.UICR,
            ACL: p.ACL,
            POWER: p.POWER,
            CLOCK: p.CLOCK,
            RADIO: p.RADIO,

            UART0: p.UART0,
            SPIM0: p.SPIM0,
            SPIS0: p.SPIS0,
            TWIM0: p.TWIM0,
            TWIS0: p.TWIS0,
            SPI0: p.SPI0,
            TWI0: p.TWI0,
            SPIM1: p.SPIM1,
            SPIS1: p.SPIS1,
            TWIM1: p.TWIM1,
            TWIS1: p.TWIS1,
            SPI1: p.SPI1,
            TWI1: p.TWI1,
            NFCT: p.NFCT,
            GPIOTE: p.GPIOTE,
            SAADC: p.SAADC,
            TIMER0: p.TIMER0,
            TIMER1: p.TIMER1,
            TIMER2: p.TIMER2,
            RTC0: p.RTC0,
            TEMP: p.TEMP,
            RNG: p.RNG,
            ECB: p.ECB,
            CCM: p.CCM,
            AAR: p.AAR,
            WDT: p.WDT,
            RTC1: p.RTC1,
            QDEC: p.QDEC,
            COMP: p.COMP,
            LPCOMP: p.LPCOMP,
            SWI0: p.SWI0,
            EGU0: p.EGU0,
            SWI1: p.SWI1,
            EGU1: p.EGU1,
            SWI2: p.SWI2,
            EGU2: p.EGU2,
            SWI3: p.SWI3,
            EGU3: p.EGU3,
            SWI4: p.SWI4,
            EGU4: p.EGU4,
            SWI5: p.SWI5,
            EGU5: p.EGU5,
            TIMER3: p.TIMER3,
            TIMER4: p.TIMER4,
            PWM0: p.PWM0,
            PDM: p.PDM,
            NVMC: p.NVMC,
            PPI: p.PPI,
            MWU: p.MWU,
            PWM1: p.PWM1,
            PWM2: p.PWM2,
            RTC2: p.RTC2,
            I2S: p.I2S,
        }
    }
}

/// The nRF52 pins that are available on the Pitaya Go
#[allow(non_snake_case)]
pub struct Pins {
    _RESET: p0::P0_19<Disconnected>,
    pub P13: p0::P0_13<Disconnected>,
    pub P14: p0::P0_14<Disconnected>,
    pub P15: p0::P0_15<Disconnected>,
    pub P16: p0::P0_16<Disconnected>,
    pub P17: p0::P0_17<Disconnected>,
    pub P20: p0::P0_20<Disconnected>,
    pub P21: p0::P0_21<Disconnected>,
    pub P22: p0::P0_22<Disconnected>,
    pub P23: p0::P0_23<Disconnected>,
    pub P24: p0::P0_24<Disconnected>,
    pub P25: p0::P0_25<Disconnected>,
    pub A1: p0::P0_03<Disconnected>,
    pub A2: p0::P0_04<Disconnected>,
    pub A3: p0::P0_05<Disconnected>,
    pub A4: p0::P0_28<Disconnected>,
    pub A5: p0::P0_29<Disconnected>,
    pub A6: p0::P0_30<Disconnected>,
    pub A7: p0::P0_31<Disconnected>,
}

/// The LEDs on the Pitaya Go board
pub struct Leds {
    pub led_r: Led,
    pub led_g: Led,
    pub led_b: Led,
}

/// An LED on the Pitaya Go board
pub struct Led(Pin<Output<PushPull>>);

impl Led {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Led(pin.into_push_pull_output(Level::High))
    }

    /// Release the inner Pin to be used directly
    pub fn release(self) -> Pin<Output<PushPull>> {
        self.0
    }

    /// Enable the LED
    pub fn enable(&mut self) {
        self.0.set_low().unwrap()
    }

    /// Disable the LED
    pub fn disable(&mut self) {
        self.0.set_high().unwrap()
    }
}

/// The Buttons on the Pitaya Go board
pub struct Buttons {
    pub button_1: Button,
}

/// A Button on the Pitaya Go board
pub struct Button(Pin<Input<PullUp>>);

impl Button {
    fn new<Mode>(pin: Pin<Mode>) -> Self {
        Button(pin.into_pullup_input())
    }

    /// Release the inner Pin to be used directly
    pub fn release(self) -> Pin<Input<PullUp>> {
        self.0
    }

    /// Button is pressed
    pub fn is_pressed(&self) -> bool {
        self.0.is_low().unwrap()
    }

    /// Button is released
    pub fn is_released(&self) -> bool {
        self.0.is_high().unwrap()
    }
}

/// The NFC pins on the Pitaya Go board
pub struct NFC {
    /// Pitaya Go: NFC1, nRF52: P0.09
    pub nfc_1: p0::P0_09<Disconnected>,

    /// Pitaya Go: NFC2, nRF52: P0.10
    pub nfc_2: p0::P0_10<Disconnected>,
}
