use crate::msg::Message;

pub fn new(payload: Vec<u8>) -> Message {
    let mut buf = payload.clone();

    let stop_id = buf[0];

    let title_len = buf[1] as usize;
    let phoneme_len = buf[2] as usize;

    // skip over bytes
    buf = buf[3..].to_vec();

    let title = String::from_utf8(buf.drain(..title_len).collect()).unwrap_or(String::new());
    let phoneme = String::from_utf8(buf.drain(..phoneme_len).collect()).unwrap_or(String::new());

    let zero_msg_len = buf[0] as usize;
    buf = buf[1..].to_vec();
    let zero_msg = String::from_utf8(buf.drain(..zero_msg_len).collect()).unwrap_or(String::new());

    let tag_len = buf[0] as usize;
    buf = buf[1..].to_vec();
    let tag = String::from_utf8(buf.drain(..tag_len).collect()).unwrap_or(String::new());

    let md5_len = buf[0] as usize;
    buf = buf[1..].to_vec();
    let md5 = String::from_utf8(buf.drain(..md5_len).collect()).unwrap_or(String::new());

    let url_len = buf[0] as usize;
    buf = buf[1..].to_vec();
    let url = String::from_utf8(buf.drain(..url_len).collect()).unwrap_or(String::new());

    Message::StopCfg {
        stop_id,
        title,
        phoneme,
        route_tag: tag,
        snd_md5: md5,
        snd_url: url,
        zero_countdown_msg: zero_msg,
    }
}

pub fn new_ack(payload: Vec<u8>) -> Message {
    Message::AckStopCfg {
        stop_id: payload[0],
        error: payload[1],
    }
}

pub fn new_clear() -> Message {
    Message::ClearStopCfg
}

pub fn new_clear_ack() -> Message {
    Message::AckClearStopCfg
}
