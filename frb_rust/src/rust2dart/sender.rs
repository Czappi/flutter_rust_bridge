use crate::generalized_isolate::Channel;
use crate::generalized_isolate::IntoDart;

#[derive(Clone)]
pub struct Rust2DartSender {
    pub(crate) channel: Channel,
}

impl Rust2DartSender {
    pub fn new(channel: Channel) -> Self {
        Rust2DartSender { channel }
    }

    pub fn send(&self, msg: impl IntoDart) -> bool {
        log::warn!("hi Rust2DartSender.send BEFORE");
        let ans = self.channel.post(msg);
        log::warn!("hi Rust2DartSender.send AFTER ans={ans}");
        ans
    }
}
