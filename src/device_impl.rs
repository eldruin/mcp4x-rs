//! Device implementation

use crate::{ic, interface, private, Channel, Command, Error, Mcp4x};
use core::marker::PhantomData;

#[doc(hidden)]
pub trait CheckChannel<CommE, PinE>: private::Sealed {
    fn check_if_channel_is_appropriate(channel: Channel) -> Result<(), Error<CommE, PinE>>;
}

impl<CommE, PinE> CheckChannel<CommE, PinE> for ic::Mcp41x {
    fn check_if_channel_is_appropriate(channel: Channel) -> Result<(), Error<CommE, PinE>> {
        if channel == Channel::Ch0 || channel == Channel::All {
            Ok(())
        } else {
            Err(Error::WrongChannel)
        }
    }
}

impl<CommE, PinE> CheckChannel<CommE, PinE> for ic::Mcp42x {
    fn check_if_channel_is_appropriate(_: Channel) -> Result<(), Error<CommE, PinE>> {
        Ok(())
    }
}

impl<DI, IC, CommE, PinE> Mcp4x<DI, IC>
where
    DI: interface::WriteCommand<Error = Error<CommE, PinE>>,
    IC: CheckChannel<CommE, PinE>,
{
    /// Set a channel to a position.
    ///
    /// Will return `Error::WrongChannel` if the channel provided is not available
    /// on the device.
    pub fn set_position(
        &mut self,
        channel: Channel,
        position: u8,
    ) -> Result<(), Error<CommE, PinE>> {
        IC::check_if_channel_is_appropriate(channel)?;
        let cmd = Command::SetPosition(channel, position);
        self.iface
            .write_command(cmd.get_command_byte(), cmd.get_data_byte())
    }

    /// Shutdown a channel.
    ///
    /// Will return `Error::WrongChannel` if the channel provided is not available
    /// on the device.
    pub fn shutdown(&mut self, channel: Channel) -> Result<(), Error<CommE, PinE>> {
        IC::check_if_channel_is_appropriate(channel)?;
        let cmd = Command::Shutdown(channel);
        self.iface
            .write_command(cmd.get_command_byte(), cmd.get_data_byte())
    }
}

impl<SPI, CS> Mcp4x<interface::SpiInterface<SPI, CS>, ic::Mcp41x> {
    /// Create new MCP41x device instance
    pub fn new_mcp41x(spi: SPI, chip_select: CS) -> Self {
        Mcp4x {
            iface: interface::SpiInterface {
                spi,
                cs: chip_select,
            },
            _ic: PhantomData,
        }
    }

    /// Destroy driver instance, return SPI bus instance and CS output pin.
    pub fn destroy_mcp41x(self) -> (SPI, CS) {
        (self.iface.spi, self.iface.cs)
    }
}

impl<SPI, CS> Mcp4x<interface::SpiInterface<SPI, CS>, ic::Mcp42x> {
    /// Create new MCP42x device instance
    pub fn new_mcp42x(spi: SPI, chip_select: CS) -> Self {
        Mcp4x {
            iface: interface::SpiInterface {
                spi,
                cs: chip_select,
            },
            _ic: PhantomData,
        }
    }

    /// Destroy driver instance, return SPI bus instance and CS output pin.
    pub fn destroy_mcp42x(self) -> (SPI, CS) {
        (self.iface.spi, self.iface.cs)
    }
}
