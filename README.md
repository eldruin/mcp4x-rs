# Rust MCP4x digital potentiometer (digipot) driver

[![crates.io](https://img.shields.io/crates/v/mcp4x.svg)](https://crates.io/crates/mcp4x)
[![Docs](https://docs.rs/mcp4x/badge.svg)](https://docs.rs/mcp4x)
[![Build Status](https://travis-ci.org/eldruin/mcp4x-rs.svg?branch=master)](https://travis-ci.org/eldruin/mcp4x-rs)
[![Coverage Status](https://coveralls.io/repos/github/eldruin/mcp4x-rs/badge.svg?branch=master)](https://coveralls.io/github/eldruin/mcp4x-rs?branch=master)
![Maintenance Intention](https://img.shields.io/badge/maintenance-actively--developed-brightgreen.svg)

This is a platform-agnostic Rust driver for the MCP41xxx and MCP42xxx SPI
digital potentiometers (digipot), based on the [`embedded-hal`] traits.

[`embedded-hal`]: https://github.com/rust-embedded/embedded-hal

This driver allows you to:
- Set a channel to a position.
- Shutdown a channel.

## The devices
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

Datasheet:
- [MCP41XXX/MCP42XXX](http://ww1.microchip.com/downloads/en/DeviceDoc/11195c.pdf)

## License

Licensed under either of

 * Apache License, Version 2.0 ([LICENSE-APACHE](LICENSE-APACHE) or
   http://www.apache.org/licenses/LICENSE-2.0)
 * MIT license ([LICENSE-MIT](LICENSE-MIT) or
   http://opensource.org/licenses/MIT) at your option.

### Contributing

Unless you explicitly state otherwise, any contribution intentionally submitted
for inclusion in the work by you, as defined in the Apache-2.0 license, shall
be dual licensed as above, without any additional terms or conditions.

