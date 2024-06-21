use embedded_hal_mock::eh1::spi::{Mock as SpiMock, Transaction as SpiTrans};
use mcp4x::{ic, interface, Channel, Error, Mcp4x};

macro_rules! device_support {
    ($create:ident, $destroy:ident, $ic:ident) => {
        pub fn $create(
            transactions: &[SpiTrans<u8>],
        ) -> Mcp4x<interface::SpiInterface<SpiMock<u8>>, ic::$ic> {
            let wrapped: Vec<SpiTrans<u8>> = transactions
                .iter()
                .flat_map(|trans| {
                    [
                        SpiTrans::transaction_start(),
                        trans.clone(),
                        SpiTrans::transaction_end(),
                    ]
                })
                .collect();
            Mcp4x::$create(SpiMock::new(&wrapped))
        }

        pub fn $destroy(dev: Mcp4x<interface::SpiInterface<SpiMock<u8>>, ic::$ic>) {
            dev.$destroy().done();
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
            let trans = [SpiTrans::write_vec(vec![$cmd, $expected_value])];
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

    fn assert_wrong_channel<T, CommE>(result: &Result<T, Error<CommE>>) {
        match result {
            Err(Error::WrongChannel) => (),
            _ => panic!("Wrong channel not reported."),
        }
    }

    #[test]
    fn wrong_channel_matches() {
        assert_wrong_channel::<(), ()>(&Err(Error::WrongChannel));
    }

    #[should_panic]
    #[test]
    fn wrong_channel_can_fail() {
        assert_wrong_channel::<(), ()>(&Ok(()));
    }

    #[test]
    fn shutdown_cannot_provide_invalid_channel_ch1() {
        let mut dev = new_mcp41x(&[]);
        assert_wrong_channel(&dev.shutdown(Channel::Ch1));
        dev.destroy_mcp41x().done();
    }

    #[test]
    fn set_position_cannot_provide_invalid_channel() {
        let mut dev = new_mcp41x(&[]);
        assert_wrong_channel(&dev.set_position(Channel::Ch1, 0));
        dev.destroy_mcp41x().done();
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
