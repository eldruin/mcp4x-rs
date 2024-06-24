use embedded_hal_bus::spi::ExclusiveDevice;
use linux_embedded_hal::{Delay, SpidevBus, SysfsPin};
use mcp4x::{Channel, Mcp4x};

fn main() {
    let spi = SpidevBus::open("/dev/spidev0.0").unwrap();
    let chip_select = SysfsPin::new(25);
    let dev = ExclusiveDevice::new(spi, chip_select, Delay);
    let mut mcp41x = Mcp4x::new_mcp41x(dev);
    mcp41x.set_position(Channel::Ch0, 50).unwrap();
    // Get SPI device and CS pin back
    let _spi = mcp41x.destroy_mcp41x();
}
