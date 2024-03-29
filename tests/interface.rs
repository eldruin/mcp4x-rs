use embedded_hal_mock::pin::{Mock as PinMock, State as PinState, Transaction as PinTrans};
use embedded_hal_mock::spi::{Mock as SpiMock, Transaction as SpiTrans};
use mcp4x::{ic, interface, Channel, Error, Mcp4x};

macro_rules! device_support {
    ($create:ident, $destroy:ident, $ic:ident) => {
        pub fn $create(
            transactions: &[SpiTrans],
        ) -> Mcp4x<interface::SpiInterface<SpiMock, PinMock>, ic::$ic> {
            let pin_transactions: Vec<PinTrans> = transactions
                .iter()
                .flat_map(|_| [PinTrans::set(PinState::Low), PinTrans::set(PinState::High)])
                .collect();
            Mcp4x::$create(SpiMock::new(transactions), PinMock::new(&pin_transactions))
        }

        pub fn $destroy(dev: Mcp4x<interface::SpiInterface<SpiMock, PinMock>, ic::$ic>) {
            dev.$destroy().0.done();
        }
    };
}

device_support!(new_mcp41x, destroy_mcp41x, Mcp41x);
device_support!(new_mcp42x, destroy_mcp42x, Mcp42x);

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
        set_position_all,
        set_position,
        new_mcp41x,
        destroy_mcp41x,
        0b0001_0011,
        50,
        Channel::All,
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
    test!(
        shutdown_all,
        shutdown,
        new_mcp41x,
        destroy_mcp41x,
        0b0010_0011,
        0,
        Channel::All
    );

    fn assert_wrong_channel<T, CommE, PinE>(result: &Result<T, Error<CommE, PinE>>) {
        match result {
            Err(Error::WrongChannel) => (),
            _ => panic!("Wrong channel not reported."),
        }
    }

    #[test]
    fn wrong_channel_matches() {
        assert_wrong_channel::<(), (), ()>(&Err(Error::WrongChannel));
    }

    #[should_panic]
    #[test]
    fn wrong_channel_can_fail() {
        assert_wrong_channel::<(), (), ()>(&Ok(()));
    }

    #[test]
    fn shutdown_cannot_provide_invalid_channel_ch1() {
        let mut dev = new_mcp41x(&[]);
        assert_wrong_channel(&dev.shutdown(Channel::Ch1));
        dev.destroy_mcp41x().0.done();
    }

    #[test]
    fn set_position_cannot_provide_invalid_channel() {
        let mut dev = new_mcp41x(&[]);
        assert_wrong_channel(&dev.set_position(Channel::Ch1, 0));
        dev.destroy_mcp41x().0.done();
    }
}

mod mcp42x {
    use super::*;

    test!(
        set_position_ch0,
        set_position,
        new_mcp42x,
        destroy_mcp42x,
        0b0001_0001,
        50,
        Channel::Ch0,
        50
    );

    test!(
        set_position_ch1,
        set_position,
        new_mcp42x,
        destroy_mcp42x,
        0b0001_0010,
        50,
        Channel::Ch1,
        50
    );

    test!(
        set_position_all,
        set_position,
        new_mcp42x,
        destroy_mcp42x,
        0b0001_0011,
        50,
        Channel::All,
        50
    );

    test!(
        shutdown_ch0,
        shutdown,
        new_mcp42x,
        destroy_mcp42x,
        0b0010_0001,
        0,
        Channel::Ch0
    );

    test!(
        shutdown_ch1,
        shutdown,
        new_mcp42x,
        destroy_mcp42x,
        0b0010_0010,
        0,
        Channel::Ch1
    );

    test!(
        shutdown_all,
        shutdown,
        new_mcp42x,
        destroy_mcp42x,
        0b0010_0011,
        0,
        Channel::All
    );
}
