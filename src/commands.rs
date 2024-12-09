//! Commands

use crate::Channel;

pub enum Command {
    /// Set a channel to a position
    SetPosition(Channel, u8),
    /// Shutdown channel
    Shutdown(Channel),
}

impl Command {
    pub(crate) fn get_command_byte(&self) -> u8 {
        match *self {
            Command::SetPosition(channel, _) => 0b0001_0000 | channel.get_bits(),
            Command::Shutdown(channel) => 0b0010_0000 | channel.get_bits(),
        }
    }
    pub(crate) fn get_data_byte(&self) -> u8 {
        match *self {
            Command::SetPosition(_, position) => position,
            Command::Shutdown(_) => 0,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    macro_rules! set_position_cmd {
        ($name:ident, $channel:ident, $position:expr, $expected_cmd:expr) => {
            #[test]
            fn $name() {
                let cmd = Command::SetPosition(Channel::$channel, $position);
                assert_eq!($expected_cmd, cmd.get_command_byte());
                assert_eq!($position, cmd.get_data_byte());
            }
        };
    }

    set_position_cmd!(can_set_position_ch0, Ch0, 127, 0b0001_0001);
    set_position_cmd!(can_set_position_ch1, Ch1, 127, 0b0001_0010);

    macro_rules! shutdown {
        ($name:ident, $channel:ident, $expected_cmd:expr) => {
            #[test]
            fn $name() {
                let cmd = Command::Shutdown(Channel::$channel);
                assert_eq!($expected_cmd, cmd.get_command_byte());
                assert_eq!(0, cmd.get_data_byte());
            }
        };
    }

    shutdown!(can_shutdown_ch_0, Ch0, 0b0010_0001);
    shutdown!(can_shutdown_ch_1, Ch1, 0b0010_0010);
}
