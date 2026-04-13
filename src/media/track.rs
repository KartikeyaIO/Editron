use crate::media::video::TimeStamp;

#[derive(Debug, Clone, PartialEq)]
pub struct AudioFrame {
    pub time: TimeStamp,
    pub data: Vec<Vec<f32>>, // [channel_index][sample_index]
}

#[derive(Debug, Clone)]
pub struct Track {
    sample_rate: u32,
    channels: u16,
    buffer: Vec<AudioFrame>,
}

#[derive(Debug)]
pub enum TrackError {
    MixingIsNotPossible,
}

// ── Constructors & Accessors ─────────────────────────────────────────────────

impl Track {
    pub fn new(sample_rate: u32, channels: u16, buffer: Vec<AudioFrame>) -> Self {
        Self {
            sample_rate,
            channels,
            buffer,
        }
    }

    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    pub fn channels(&self) -> u16 {
        self.channels
    }
    pub fn buffer(&self) -> &[AudioFrame] {
        &self.buffer
    }
}

impl Track {
    /// Apply a gain (in dB) to every sample in every frame.
    pub fn gain(&mut self, db: f32) {
        let factor = 10.0_f32.powf(db / 20.0);
        for frame in &mut self.buffer {
            for channel in &mut frame.data {
                for sample in channel {
                    *sample *= factor;
                }
            }
        }
    }

    /// Mix two tracks sample-by-sample. Both must have identical shape.
    pub fn mix(a: &Track, b: &Track) -> Result<Track, TrackError> {
        if a.channels != b.channels
            || a.sample_rate != b.sample_rate
            || a.buffer.len() != b.buffer.len()
        {
            return Err(TrackError::MixingIsNotPossible);
        }

        let buffer = a
            .buffer
            .iter()
            .zip(b.buffer.iter())
            .map(|(fa, fb)| {
                let data = fa
                    .data
                    .iter()
                    .zip(fb.data.iter())
                    .map(|(ca, cb)| ca.iter().zip(cb.iter()).map(|(sa, sb)| sa + sb).collect())
                    .collect();

                AudioFrame {
                    time: fa.time.clone(),
                    data,
                }
            })
            .collect();

        Ok(Track::new(a.sample_rate, a.channels, buffer))
    }

    /// Flatten all frames/channels into an interleaved f32 PCM stream.
    pub fn to_pcm_f32(&self) -> Vec<f32> {
        self.interleaved().map(|s| s.clamp(-1.0, 1.0)).collect()
    }

    /// Flatten all frames/channels into an interleaved i16 PCM stream.
    pub fn to_pcm_i16(&self) -> Vec<i16> {
        self.interleaved()
            .map(|s| (s.clamp(-1.0, 1.0) * i16::MAX as f32) as i16)
            .collect()
    }

    /// Normalize the track in-place so the loudest sample hits 1.0 or -1.0.
    /// Returns false if the buffer is empty or already silent.
    pub fn normalize(&mut self) -> bool {
        let max = self
            .buffer
            .iter()
            .flat_map(|f| f.data.iter().flat_map(|ch| ch.iter()))
            .map(|s| s.abs())
            .max_by(f32::total_cmp);

        match max {
            Some(peak) if peak > 0.0 => {
                for frame in &mut self.buffer {
                    for channel in &mut frame.data {
                        for sample in channel {
                            *sample /= peak;
                        }
                    }
                }
                true
            }
            _ => false,
        }
    }

    /// Iterator over every sample, interleaved by channel across all frames.
    fn interleaved(&self) -> impl Iterator<Item = f32> + '_ {
        self.buffer.iter().flat_map(|frame| {
            // For each frame, yield samples interleaved
            let len = frame.data.first().map_or(0, |ch| ch.len());
            (0..len).flat_map(move |i| frame.data.iter().map(move |ch| ch[i]))
        })
    }
}
