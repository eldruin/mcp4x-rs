//! Device implementation

use crate::{ic, interface, private, Channel, Command, Error, Mcp4x};
use core::marker::PhantomData;

#[doc(hidden)]
pub trait CheckParameters<CommE>: private::Sealed {
    fn check_if_channel_is_appropriate(channel: Channel) -> Result<(), Error<CommE>>;
    fn check_if_position_is_appropriate(position: u8) -> Result<(), Error<CommE>>;
}

impl<CommE> CheckParameters<CommE> for ic::Mcp401x {
    fn check_if_channel_is_appropriate(channel: Channel) -> Result<(), Error<CommE>> {
        if channel == Channel::Ch0 || channel == Channel::All {
            Ok(())
        } else {
            Err(Error::WrongChannel)
        }
    }

    fn check_if_position_is_appropriate(position: u8) -> Result<(), Error<CommE>> {
        if position <= 127 {
            Ok(())
        } else {
            Err(Error::OutOfBounds)
        }
    }
}

impl<CommE> CheckParameters<CommE> for ic::Mcp41x {
    fn check_if_channel_is_appropriate(channel: Channel) -> Result<(), Error<CommE>> {
        if channel == Channel::Ch0 || channel == Channel::All {
            Ok(())
        } else {
            Err(Error::WrongChannel)
        }
    }

    fn check_if_position_is_appropriate(_position: u8) -> Result<(), Error<CommE>> {
        Ok(())
    }
}

impl<CommE> CheckParameters<CommE> for ic::Mcp42x {
    fn check_if_channel_is_appropriate(_: Channel) -> Result<(), Error<CommE>> {
        Ok(())
    }

    fn check_if_position_is_appropriate(_position: u8) -> Result<(), Error<CommE>> {
        Ok(())
    }
}

impl<DI, IC, CommE> Mcp4x<DI, IC>
where
    DI: interface::WriteCommand<Error = Error<CommE>>,
    IC: CheckParameters<CommE>,
{
    /// Set a channel to a position.
    ///
    /// Will return `Error::WrongChannel` if the channel provided is not available
    /// on the device.
    pub fn set_position(&mut self, channel: Channel, position: u8) -> Result<(), Error<CommE>> {
        IC::check_if_channel_is_appropriate(channel)?;
        IC::check_if_position_is_appropriate(position)?;
        self.iface
            .write_command(Command::SetPosition(channel, position))
    }

    /// Shutdown a channel.
    ///
    /// Will return `Error::WrongChannel` if the channel provided is not available
    /// on the device.
    pub fn shutdown(&mut self, channel: Channel) -> Result<(), Error<CommE>> {
        IC::check_if_channel_is_appropriate(channel)?;
        self.iface.write_command(Command::Shutdown(channel))
    }
}

impl<I2C> Mcp4x<interface::I2cInterface<I2C>, ic::Mcp401x> {
    /// Create new MCP401x device instance
    pub fn new_mcp401x(i2c: I2C) -> Self {
        Mcp4x {
            iface: interface::I2cInterface { i2c },
            _ic: PhantomData,
        }
    }

    /// Destroy driver instance, return I2C bus instance.
    pub fn destroy_mcp401x(self) -> I2C {
        self.iface.i2c
    }
}

impl<SPI> Mcp4x<interface::SpiInterface<SPI>, ic::Mcp41x> {
    /// Create new MCP41x device instance
    pub fn new_mcp41x(spi: SPI) -> Self {
        Mcp4x {
            iface: interface::SpiInterface { spi },
            _ic: PhantomData,
        }
    }

    /// Destroy driver instance, return SPI bus instance and CS output pin.
    pub fn destroy_mcp41x(self) -> SPI {
        self.iface.spi
    }
}

impl<SPI> Mcp4x<interface::SpiInterface<SPI>, ic::Mcp42x> {
    /// Create new MCP42x device instance
    pub fn new_mcp42x(spi: SPI) -> Self {
        Mcp4x {
            iface: interface::SpiInterface { spi },
            _ic: PhantomData,
        }
    }

    /// Destroy driver instance, return SPI bus instance and CS output pin.
    pub fn destroy_mcp42x(self) -> SPI {
        self.iface.spi
    }
}
