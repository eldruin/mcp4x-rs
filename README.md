# Rust MCP4x digital potentiometer (digipot) driver

[![crates.io](https://img.shields.io/crates/v/mcp4x.svg)](https://crates.io/crates/mcp4x)
[![Docs](https://docs.rs/mcp4x/badge.svg)](https://docs.rs/mcp4x)
![Minimum Supported Rust Version](https://img.shields.io/badge/rustc-1.62+-blue.svg)
[![Build Status](https://github.com/eldruin/mcp4x-rs/workflows/Build/badge.svg)](https://github.com/eldruin/mcp4x-rs/actions?query=workflow%3ABuild)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/mcp4x-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/mcp4x-rs?branch=master)

This is a platform-agnostic Rust driver for the MCP41xxx and MCP42xxx SPI
digital potentiometers (digipot), based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Set a channel (or all of them) to a position. See: `set_position()`.
- Shutdown a channel (or all of them). See: `shutdown()`.

## The devices

### SPI
The MCP41XXX and MCP42XXX devices are 256-position, digital potentiometers
available in 10 kΩ, 50 kΩ and 100 kΩ resistance versions. The MCP41XXX is
a single-channel device and is offered in an 8-pin PDIP or SOIC package.
The MCP42XXX contains two independent channels in a 14-pin PDIP, SOIC or
TSSOP package. The wiper position of the MCP41XXX/42XXX varies linearly
and is controlled via an industry-standard SPI interface.

The devices consume <1 μA during static operation. A software shutdown
feature is provided that disconnects the "A" terminal from the resistor
stack and simultaneously connects the wiper to the "B" terminal.
In addition, the dual MCP42XXX has a SHDN pin that performs the same
function in hardware. During shutdown mode, the contents of the wiper
register can be changed and the potentiometer returns from shutdown to the
new value. The wiper is reset to the mid-scale position (80h) upon
power-up. The RS (reset) pin implements a hardware reset and also returns
the wiper to mid-scale.

The MCP42XXX SPI interface includes both the SI and SO pins, allowing
daisy-chaining of multiple devices. Channel-to-channel resistance matching
on the MCP42XXX varies by less than 1%.

This driver should be compatible at least with the devices: MCP41010, MCP41050,
MCP41100, MCP42010, MCP42050 and MCP42100.

Datasheet:
- [MCP41XXX/MCP42XXX](http://ww1.microchip.com/downloads/en/DeviceDoc/11195c.pdf)

### I2C

The MCP40XX device is a 128-position, digital potentiometer
available in 5 kΩ, 10 kΩ, 50 kΩ and 100 kΩ resistance versions. The MCP40XX
is a single-channel device and is offered in an 6-pin SC70 package.
The wiper position of the MCP40XX varies linearly and is controlled via an
industry-standard I2C interface.

Datasheet:
- [MCP4017/18/19](https://ww1.microchip.com/downloads/en/DeviceDoc/22147a.pdf)

## Usage

To use this driver, import this crate and an `embedded_hal` implementation,
then instantiate the appropriate device.
In the following examples an instance of the device MCP41x will be created
as an example. Other devices can be created with similar methods like:
`Mcp4x::new_mcp42x(...)`.

Please find additional examples using hardware in this repository: [driver-examples]

[driver-examples]: https://github.com/eldruin/driver-examples

```rust
use mcp4x::{Channel, Mcp4x};
use embedded_hal_bus::spi::ExclusiveDevice;
use linux_embedded_hal::{Delay, SpidevBus, SysfsPin};

fn main() {
    let spi = SpidevBus::open("/dev/spidev0.0").unwrap();
    let chip_select = SysfsPin::new(25);
    let dev = ExclusiveDevice::new(spi, chip_select, Delay);
    let mut mcp41x = Mcp4x::new_mcp41x(dev);

    mcp41x.set_position(Channel::Ch0, 50).unwrap();

    // Get SPI device back
    let _dev = mcp41x.destroy_mcp41x();
}
```

### Async support

This library support async execution by adding the feature `async` to `Cargo.toml`:

```toml
[dependencies]
mcp4x = { version = "1.1", features = ["async"] }
```

## Support

For questions, issues, feature requests like compatibility with similar devices and other changes, please file an
[issue in the github project](https://github.com/eldruin/mcp4x-rs/issues).

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT)

at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.
