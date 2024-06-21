//! SPI interface

use crate::{private, Error};
use embedded_hal::spi::SpiDevice;

/// SPI interface
#[derive(Debug, Default)]
pub struct SpiInterface<SPI> {
    pub(crate) spi: SPI,
}

/// Perform a command
pub trait WriteCommand: private::Sealed {
    /// Error type
    type Error;

    /// Command
    fn write_command(&mut self, command: u8, data: u8) -> Result<(), Self::Error>;
}

impl<SPI, E> WriteCommand for SpiInterface<SPI>
where
    SPI: SpiDevice<Error = E>,
{
    type Error = Error<E>;

    fn write_command(&mut self, command: u8, data: u8) -> Result<(), Error<E>> {
        let payload: [u8; 2] = [command, data];
        let result = self.spi.write(&payload).map_err(Error::Comm);

        result
    }
}
