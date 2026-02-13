use std::fmt;
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum PixelFormat {
    RGB24,
    Gray8,
    RGBA32,
}

impl PixelFormat {
    pub fn bytes_per_pixel(self) -> usize {
        match self {
            PixelFormat::RGB24 => 3,
            PixelFormat::Gray8 => 1,
            PixelFormat::RGBA32 => 4,
        }
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub struct Timestamp {
    pub micros: u64,
}

impl Timestamp {
    pub fn from_micros(us: u64) -> Self {
        Self { micros: us }
    }

    pub fn from_seconds(s: f64) -> Self {
        Self {
            micros: (s * 1_000_000.0).round() as u64,
        }
    }
}

#[derive(Debug)]
pub struct Frame {
    width: u32,
    height: u32,
    format: PixelFormat,
    data: Vec<u8>,
}

#[derive(Debug)]
pub enum FrameError {
    InvalidFrameSize,
}
impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameError::InvalidFrameSize => {
                write!(f, "Invalid frame buffer size")
            }
        }
    }
}

impl std::error::Error for FrameError {}

impl Frame {
    pub fn new(
        width: u32,
        height: u32,
        format: PixelFormat,
        data: Vec<u8>,
    ) -> Result<Self, FrameError> {
        let expected_len = width as usize * height as usize * format.bytes_per_pixel();

        if data.len() == expected_len {
            Ok(Self {
                width,
                height,
                format,
                data,
            })
        } else {
            Err(FrameError::InvalidFrameSize)
        }
    }
    pub fn brightness(&mut self, delta: i16) {
        for pixel in &mut self.data {
            let value = *pixel as i16 + delta;

            let clamped = value.clamp(0, 255);

            *pixel = clamped as u8;
        }
    }

    pub fn format(&self) -> PixelFormat {
        self.format
    }
    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn data(&self) -> &[u8] {
        &self.data
    }
}
