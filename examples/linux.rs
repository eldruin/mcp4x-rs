extern crate embedded_hal;
extern crate linux_embedded_hal;
extern crate mcp4x;

use mcp4x::{Channel, Mcp4x};
use linux_embedded_hal::{Pin, Spidev};

fn main() {
    let spi = Spidev::open("/dev/spidev0.0").unwrap();
    let chip_select = Pin::new(25);
    let mut mcp41x = Mcp4x::new_mcp41x(spi, chip_select);
    mcp41x.set_position(Channel::Ch0, 50).unwrap();
    // Get SPI device and CS pin back
    let (_spi, _chip_select) = mcp41x.destroy_mcp41x();
}
