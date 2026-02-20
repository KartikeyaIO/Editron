pub struct Track {
    sample_rate: u32,
    channels: u16,
    buffer: Vec<f32>,
}

/// Accessors
impl Track {
    pub fn sample_rate(&self) -> &u32 {
        &self.sample_rate
    }
    pub fn channels(&self) -> &u16 {
        &self.channels
    }
    pub fn buffer(&self) -> &Vec<f32> {
        &self.buffer
    }
}

impl Track {}
