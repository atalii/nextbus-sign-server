use crate::msg::Message;

pub fn new_get(payload: Vec<u8>) -> Message {
    Message::GetCfgParam { param: payload[0] }
}

pub fn new_get_ack(payload: Vec<u8>) -> Message {
    Message::AckGetCfgParam {
        param: payload[0],
        error: payload[1],
        value: payload[2],
    }
}

pub fn new_set(payload: Vec<u8>) -> Message {
    Message::SetCfgParam {
        param: payload[0],
        value: payload[1],
    }
}

pub fn new_set_ack(payload: Vec<u8>) -> Message {
    Message::AckSetCfgParam {
        param: payload[0],
        error: payload[1],
        value: payload[2],
    }
}

pub fn new_reset() -> Message {
    Message::ResetCfgParams
}

pub fn new_reset_ack() -> Message {
    Message::AckResetCfgParams
}
