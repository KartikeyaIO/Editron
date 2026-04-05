use crate::media::frame::Frame;

#[derive(Clone, Copy, Debug, PartialEq, Eq, PartialOrd, Ord)]
pub struct TimeStamp {
    pub value: u64,
    pub timescale: u32, // units per second
}

pub struct VideoFrame {
    frame: Frame,
    pts: TimeStamp,
}
pub struct Video {}
