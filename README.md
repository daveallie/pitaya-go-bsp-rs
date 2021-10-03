# Board Support Package for the Makerdiary Pitaya Go (nRF52840)

This crate is a Board Support Package (BSP). It wraps the HAL crate (nrf52840-hal) for the on-board nRF52840, and provides high level wrappers for the
onboard features.

This BSP assumes you are not using a bootloader running in non-secure mode.

## Usage

You will require the `thumbv7em-none-eabihf` target installed. To build one of these examples:

```console
$ rustup target add thumbv7em-none-eabihf
$ git clone https://github.com/daveallie/pitaya-go-bsp-rs.git
$ cd 
$ cargo build --target=thumbv7em-none-eabihf --example blinky
```

To use in your own application, add as a dependency and call the
`Board::init()` function.

## Licence

MIT license ([LICENSE-MIT](LICENSE-MIT) or http://opensource.org/licenses/MIT)
