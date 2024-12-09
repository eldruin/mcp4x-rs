//! SPI interface

#[cfg(feature = "async")]
use core::future;

use crate::{private, Command, Error};
#[cfg(not(feature = "async"))]
use embedded_hal::i2c::I2c;
#[cfg(feature = "async")]
use embedded_hal_async::i2c::I2c;

#[cfg(not(feature = "async"))]
use embedded_hal::spi::SpiDevice;
#[cfg(feature = "async")]
use embedded_hal_async::spi::SpiDevice;

/// SPI interface
#[derive(Debug, Default)]
pub struct SpiInterface<SPI> {
    pub(crate) spi: SPI,
}

/// I2C interface
#[derive(Debug, Default)]
pub struct I2cInterface<I2C> {
    pub(crate) i2c: I2C,
}

/// Perform a command
pub trait WriteCommand: private::Sealed {
    /// Error type
    type Error;

    #[cfg(not(feature = "async"))]
    /// Command
    fn write_command(&mut self, command: Command) -> Result<(), Self::Error>;
    #[cfg(feature = "async")]
    /// Command
    fn write_command(
        &mut self,
        command: Command,
    ) -> impl future::Future<Output = Result<(), Self::Error>>;
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")),),
    async(feature = "async"),
    keep_self
)]
impl<SPI, E> WriteCommand for SpiInterface<SPI>
where
    SPI: SpiDevice<Error = E>,
{
    type Error = Error<E>;

    async fn write_command(&mut self, command: Command) -> Result<(), Self::Error> {
        let payload: [u8; 2] = [command.get_command_byte(), command.get_data_byte()];
        self.spi.write(&payload).await.map_err(Error::Comm)
    }
}

#[maybe_async_cfg::maybe(
    sync(cfg(not(feature = "async")),),
    async(feature = "async"),
    keep_self
)]
impl<I2C, E> WriteCommand for I2cInterface<I2C>
where
    I2C: I2c<Error = E>,
{
    type Error = Error<E>;

    async fn write_command(&mut self, command: Command) -> Result<(), Self::Error> {
        const ADDRESS: u8 = 0b0101111;
        match command {
            Command::SetPosition(_, position) => self
                .i2c
                .write(ADDRESS, &[position])
                .await
                .map_err(Error::Comm),
            Command::Shutdown(_) => Err(Error::Unsupported),
        }
    }
}
