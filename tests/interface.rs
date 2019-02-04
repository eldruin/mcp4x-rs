extern crate embedded_hal;
extern crate mcp4x;
use mcp4x::{ic, interface, Channel, Error, Mcp4x};
extern crate embedded_hal_mock as hal;
use self::hal::spi::{Mock as SpiMock, Transaction as SpiTrans};

pub struct DummyOutputPin;

impl embedded_hal::digital::OutputPin for DummyOutputPin {
    fn set_low(&mut self) {}
    fn set_high(&mut self) {}
}

pub fn new_mcp41x(
    transactions: &[SpiTrans],
) -> Mcp4x<interface::SpiInterface<SpiMock, DummyOutputPin>, ic::Mcp41x> {
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

mod mcp41x {
    use super::*;
    test!(
        set_position_ch0,
        set_position,
        new_mcp41x,
        destroy_mcp41x,
        0b0001_0001,
        50,
        Channel::Ch0,
        50
    );
    test!(
        shutdown_ch0,
        shutdown,
        new_mcp41x,
        destroy_mcp41x,
        0b0010_0001,
        0,
        Channel::Ch0
    );

    fn assert_wrong_channel<T, E>(result: Result<T, Error<E>>) {
        match result {
            Err(Error::WrongChannel) => (),
            _ => panic!("Wrong channel not reported."),
        }
    }

    #[test]
    fn wrong_channel_matches() {
        assert_wrong_channel::<(), ()>(Err(Error::WrongChannel));
    }

    #[should_panic]
    #[test]
    fn wrong_channel_can_fail() {
        assert_wrong_channel::<(), ()>(Ok(()));
    }

    #[test]
    fn shutdown_cannot_provide_invalid_channel() {
        let mut dev = new_mcp41x(&[]);
        assert_wrong_channel(dev.shutdown(Channel::Ch1));
        dev.destroy_mcp41x().0.done();
    }

    #[test]
    fn set_position_cannot_provide_invalid_channel() {
        let mut dev = new_mcp41x(&[]);
        assert_wrong_channel(dev.set_position(Channel::Ch1, 0));
        dev.destroy_mcp41x().0.done();
    }
}
