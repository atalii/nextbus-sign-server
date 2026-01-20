use crate::msg::Message;

pub fn new(payload: Vec<u8>) -> Message {
    Message::DebugMsg {
        msg: String::from_utf8(payload).expect("Invalid payload in debug message!"),
    }
}
