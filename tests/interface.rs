extern crate embedded_hal;
extern crate mcp4x;
use mcp4x::{ Channel, Mcp4x, interface, ic };
extern crate embedded_hal_mock as hal;
use self::hal::spi::{ Mock as SpiMock, Transaction as SpiTrans };

pub struct DummyOutputPin;

impl embedded_hal::digital::OutputPin for DummyOutputPin {
    fn set_low(&mut self) {}
    fn set_high(&mut self) {}
}

pub fn new_mcp41x(transactions: &[SpiTrans])
    -> Mcp4x<interface::SpiInterface<SpiMock, DummyOutputPin>, ic::Mcp41x> {
    Mcp4x::new_mcp41x(SpiMock::new(&transactions), DummyOutputPin)
}

pub fn destroy_mcp41x(dev: Mcp4x<interface::SpiInterface<SpiMock, DummyOutputPin>, ic::Mcp41x>) {
    dev.destroy_mcp41x().0.done();
}

#[macro_export]
macro_rules! test {
    ($name:ident, $method:ident, $create:ident, $destroy:ident,
        $cmd:expr, $expected_value:expr, $( $arg:expr ),* ) => {
        #[test]
        fn $name() {
            let trans = [SpiTrans::write(vec![$cmd, $expected_value])];
            let mut dev = $create(&trans);
            dev.$method($($arg),*).unwrap();
            $destroy(dev);
        }
    };
}

test!(set_position_ch0, set_position, new_mcp41x, destroy_mcp41x, 0b0001_0001, 50, Channel::Ch0, 50);
test!(shutdown_ch0, shutdown, new_mcp41x, destroy_mcp41x, 0b0010_0001, 0, Channel::Ch0);
