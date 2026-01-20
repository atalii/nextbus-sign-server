use crate::msg::Message;

pub fn new(payload: Vec<u8>) -> Message {
    Message::ContentDelete {
        content_id: u16::from_be_bytes([payload[0], payload[1]]),
    }
}
