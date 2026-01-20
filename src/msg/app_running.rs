use crate::msg::Message;

#[derive(Debug)]
pub enum AppRunningReason {
    Undiscernable,
    Powerup,
    Watchdog,
    ServerOrder,
    NewFirmware,
    NoServerContact,
    Redirected,
    DroppedConnection,
    BadAuthentication,
    FatalError,
    Unknown,
}

impl From<u8> for AppRunningReason {
    fn from(x: u8) -> Self {
        match x {
            0 => Self::Undiscernable,
            1 => Self::Powerup,
            2 => Self::Watchdog,
            3 => Self::ServerOrder,
            4 => Self::NewFirmware,
            5 => Self::NoServerContact,
            6 => Self::Redirected,
            7 => Self::DroppedConnection,
            8 => Self::BadAuthentication,
            9 => Self::FatalError,
            _ => Self::Unknown,
        }
    }
}

pub fn new(payload: Vec<u8>) -> Message {
    Message::AppRunning {
        seq_num: payload[0],
        reason: payload[1].into(),
    }
}
