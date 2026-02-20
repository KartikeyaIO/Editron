use crate::media::frame::Frame;
pub trait Filter {
    fn apply(&self, frame: Frame) -> Frame {
        frame
    }
}
