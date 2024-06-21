//! Device implementation

use crate::{ic, interface, private, Channel, Command, Error, Mcp4x};
use core::marker::PhantomData;

#[doc(hidden)]
pub trait CheckChannel<CommE>: private::Sealed {
    fn check_if_channel_is_appropriate(channel: Channel) -> Result<(), Error<CommE>>;
}

impl<CommE> CheckChannel<CommE> for ic::Mcp41x {
    fn check_if_channel_is_appropriate(channel: Channel) -> Result<(), Error<CommE>> {
        if channel == Channel::Ch0 || channel == Channel::All {
            Ok(())
        } else {
            Err(Error::WrongChannel)
        }
    }
}

impl<CommE> CheckChannel<CommE> for ic::Mcp42x {
    fn check_if_channel_is_appropriate(_: Channel) -> Result<(), Error<CommE>> {
        Ok(())
    }
}

impl<DI, IC, CommE> Mcp4x<DI, IC>
where
    DI: interface::WriteCommand<Error = Error<CommE>>,
    IC: CheckChannel<CommE>,
{
    /// Set a channel to a position.
    ///
    /// Will return `Error::WrongChannel` if the channel provided is not available
    /// on the device.
    pub fn set_position(&mut self, channel: Channel, position: u8) -> Result<(), Error<CommE>> {
        IC::check_if_channel_is_appropriate(channel)?;
        let cmd = Command::SetPosition(channel, position);
        self.iface
            .write_command(cmd.get_command_byte(), cmd.get_data_byte())
    }

    /// Shutdown a channel.
    ///
    /// Will return `Error::WrongChannel` if the channel provided is not available
    /// on the device.
    pub fn shutdown(&mut self, channel: Channel) -> Result<(), Error<CommE>> {
        IC::check_if_channel_is_appropriate(channel)?;
        let cmd = Command::Shutdown(channel);
        self.iface
            .write_command(cmd.get_command_byte(), cmd.get_data_byte())
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
