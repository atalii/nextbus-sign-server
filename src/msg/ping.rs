use crate::msg::Message;

pub fn new(payload: Vec<u8>) -> Message {
    Message::Ping {
        seq_num: payload[0],
    }
}
