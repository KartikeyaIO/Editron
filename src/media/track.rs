#[derive(Debug, Clone)]
pub struct Track {
    sample_rate: u32,
    channels: u16,
    buffer: Vec<f32>,
}

pub enum TrackError {
    InvalidTrack,
    MixingIsNotPossible,
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
        for sample in &mut self.buffer {
            *sample *= factor;
        }
    }

    pub fn mix(track1: &Track, track2: &Track) -> Result<Track, TrackError> {
        if track1.channels != track2.channels
            || track1.sample_rate != track2.sample_rate
            || track1.buffer.len() != track2.buffer.len()
        {
            return Err(TrackError::MixingIsNotPossible);
        }

        let buffer: Vec<f32> = track1
            .buffer
            .iter()
            .zip(track2.buffer.iter())
            .map(|(a, b)| a + b)
            .collect();

        Ok(Track::new(track1.sample_rate, track1.channels, buffer))
    }
    pub fn to_pcm_f32(track: &Track) -> Vec<f32> {
        track.buffer.iter().map(|&x| x.clamp(-1.0, 1.0)).collect()
    }
    pub fn to_pcm_i16(track: &Track) -> Vec<i16> {
        track
            .buffer
            .iter()
            .map(|x| (x.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
            .collect()
    }
    pub fn normalize(&mut self) -> bool {
        let max = {
            self.buffer()
                .iter()
                .map(|v| v.abs())
                .max_by(|a, b| a.total_cmp(b))
        };
        let data = &mut self.buffer;
        match max {
            Some(value) => {
                for i in 0..data.len() {
                    data[i] /= value;
                }
                return true;
            }
            None => {
                return false;
            }
        }
    }
}
