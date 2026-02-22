#[derive(Debug, Clone)]
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
    pub fn new(sr: u32, channel: u16, buffer: Vec<f32>) -> Self {
        Self {
            sample_rate: sr,
            channels: channel,
            buffer,
        }
    }
}

impl Track {
    pub fn gain(&mut self, db: f32) {
        let factor = 10.0_f32.powf(db / 20.0);
        for i in &mut self.buffer {
            *i = *i * factor;
        }
    }
}
