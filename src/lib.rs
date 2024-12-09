//! This is a platform-agnostic Rust driver for the MCP41xxx and MCP42xxx SPI
//! digital potentiometers (digipot), based on the [`embedded-hal`] traits.
//!
//! [`embedded-hal`]: https://github.com/rust-embedded/embedded-hal
//!
//! This driver allows you to:
//! - Set a channel (or all of them) to a position. See: [`set_position()`].
//! - Shutdown a channel (or all of them). See: [`shutdown()`].
//!
//! [`set_position()`]: struct.Mcp4x.html#method.set_position
//! [`shutdown()`]: struct.Mcp4x.html#method.shutdown
//!
//! ## The devices
//! The MCP41XXX and MCP42XXX devices are 256-position, digital potentiometers
//! available in 10 kΩ, 50 kΩ and 100 kΩ resistance versions. The MCP41XXX is
//! a single-channel device and is offered in an 8-pin PDIP or SOIC package.
//! The MCP42XXX contains two independent channels in a 14-pin PDIP, SOIC or
//! TSSOP package. The wiper position of the MCP41XXX/42XXX varies linearly
//! and is controlled via an industry-standard SPI interface.
//!
//! The devices consume <1 μA during static operation. A software shutdown
//! feature is provided that disconnects the "A" terminal from the resistor
//! stack and simultaneously connects the wiper to the "B" terminal.
//! In addition, the dual MCP42XXX has a SHDN pin that performs the same
//! function in hardware. During shutdown mode, the contents of the wiper
//! register can be changed and the potentiometer returns from shutdown to the
//! new value. The wiper is reset to the mid-scale position (80h) upon
//! power-up. The RS (reset) pin implements a hardware reset and also returns
//! the wiper to mid-scale.
//!
//! The MCP42XXX SPI interface includes both the SI and SO pins, allowing
//! daisy-chaining of multiple devices. Channel-to-channel resistance matching
//! on the MCP42XXX varies by less than 1%.
//!
//! This driver should be compatible at least with the devices: MCP41010, MCP41050,
//! MCP41100, MCP42010, MCP42050 and MCP42100.
//!
//! Datasheet:
//! - [MCP41XXX/MCP42XXX](http://ww1.microchip.com/downloads/en/DeviceDoc/11195c.pdf)
//!
//!
//! ## Usage examples (see also examples folder)
//!
//! To use this driver, import this crate and an `embedded_hal` implementation,
//! then instantiate the appropriate device.
//!
//! Please find additional examples using hardware in this repository: [driver-examples]
//!
//! [driver-examples]: https://github.com/eldruin/driver-examples
//!
//! ### Set channel 0 to position 125 in a MCP41x device
//!
//! ```no_run
//! use mcp4x::{Channel, Mcp4x};
//! use embedded_hal_bus::spi::ExclusiveDevice;
//! use linux_embedded_hal::{Delay, SpidevBus, SysfsPin};
//!
//! let spi = SpidevBus::open("/dev/spidev0.0").unwrap();
//! let chip_select = SysfsPin::new(25);
//! let dev = ExclusiveDevice::new(spi, chip_select, Delay);
//! let mut mcp41x = Mcp4x::new_mcp41x(dev);
//!
//! mcp41x.set_position(Channel::Ch0, 125).unwrap();
//!
//! // Get SPI device back
//! let _dev = mcp41x.destroy_mcp41x();
//! ```
//!
//! ### Set channels to positions in a MCP42x device
//!
//! ```no_run

//! use mcp4x::{Channel, Mcp4x};
//! use embedded_hal_bus::spi::ExclusiveDevice;
//! use linux_embedded_hal::{Delay, SpidevBus, SysfsPin};
//!
//! let spi = SpidevBus::open("/dev/spidev0.0").unwrap();
//! let chip_select = SysfsPin::new(25);
//! let dev = ExclusiveDevice::new(spi, chip_select, Delay);
//! let mut mcp42x = Mcp4x::new_mcp42x(dev);
//!
//! mcp42x.set_position(Channel::Ch0, 50).unwrap();
//! mcp42x.set_position(Channel::Ch1, 50).unwrap();
//! ```
//!
//! ### Set all channels to position in a MCP42x device
//!
//! ```no_run

//! use mcp4x::{Channel, Mcp4x};
//! use embedded_hal_bus::spi::ExclusiveDevice;
//! use linux_embedded_hal::{Delay, SpidevBus, SysfsPin};
//! let spi = SpidevBus::open("/dev/spidev0.0").unwrap();
//! let chip_select = SysfsPin::new(25);
//! let dev = ExclusiveDevice::new(spi, chip_select, Delay);
//! let mut mcp42x = Mcp4x::new_mcp42x(dev);
//!
//! mcp42x.set_position(Channel::All, 50).unwrap();
//! ```
//!
//! ### Shutdown a channel
//!
//! ```no_run

//! use mcp4x::{Channel, Mcp4x};
//! use embedded_hal_bus::spi::ExclusiveDevice;
//! use linux_embedded_hal::{Delay, SpidevBus, SysfsPin};
//! let spi = SpidevBus::open("/dev/spidev0.0").unwrap();
//! let chip_select = SysfsPin::new(25);
//! let dev = ExclusiveDevice::new(spi, chip_select, Delay);
//! let mut mcp42x = Mcp4x::new_mcp42x(dev);
//!
//! mcp42x.shutdown(Channel::Ch0).unwrap();
//! ```
//!

#![deny(unsafe_code)]
#![deny(missing_docs)]
#![no_std]

use core::marker::PhantomData;
use embedded_hal::spi::{Mode, MODE_0};

/// All possible errors in this crate
#[derive(Debug)]
pub enum Error<CommE> {
    /// Communication error
    Comm(CommE),
    /// Wrong channel for this device provided
    WrongChannel,
    /// Unsupported feature
    Unsupported,
    /// Parameter out of bounds
    OutOfBounds,
}

/// SPI mode
pub const MODE: Mode = MODE_0;

/// Channel selector
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Channel {
    /// Channel 0
    Ch0,
    /// Channel 1 (only for MCP42XXX devices)
    Ch1,
    /// Select all channels
    All,
}

impl Channel {
    fn get_bits(self) -> u8 {
        match self {
            Channel::Ch0 => 1,
            Channel::Ch1 => 2,
            Channel::All => 3,
        }
    }
}

/// IC markers
pub mod ic {
    /// MCP401x IC marker
    pub struct Mcp401x;

    /// MCP41x IC marker
    pub struct Mcp41x;

    /// MCP42x IC marker
    pub struct Mcp42x;
}

/// MCP4x digital potentiometer driver
#[derive(Debug, Default)]
pub struct Mcp4x<DI, IC> {
    iface: DI,
    _ic: PhantomData<IC>,
}

mod device_impl;
pub use crate::device_impl::CheckParameters;

mod commands;
use crate::commands::Command;

/// Interface
pub mod interface;

mod private {
    use super::{ic, interface};
    pub trait Sealed {}

    impl<SPI> Sealed for interface::SpiInterface<SPI> {}
    impl<I2C> Sealed for interface::I2cInterface<I2C> {}
    impl Sealed for ic::Mcp401x {}
    impl Sealed for ic::Mcp41x {}
    impl Sealed for ic::Mcp42x {}
}
