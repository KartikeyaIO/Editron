use std::fmt;

// CONFIGS
#[derive(Debug, Clone, Copy)]
pub struct Color(pub u8, pub u8, pub u8);
pub struct Pos(pub u32, pub u32);

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
/// PixelFormat is a enum which helps us decide the PixelFormat of a decoded image
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
/// Frame Type is used to store an image or a single frame from a video.
pub struct Frame {
    width: u32,
    height: u32,
    format: PixelFormat,
    data: Vec<u8>,
}

#[derive(Debug)]
pub enum FrameError {
    InvalidFrameSize,
    InvalidPixel,
}
impl fmt::Display for FrameError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FrameError::InvalidFrameSize => {
                write!(f, "Invalid frame buffer size")
            }
            FrameError::InvalidPixel => {
                write!(f, "Unable to find the pixel!")
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
    /// The brightness function is used to adjust the brightness of a Frame
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
impl Frame {
    pub fn pixel_index(&self, pos: &Pos) -> Result<usize, FrameError> {
        let Pos(x, y) = *pos;

        if x >= self.width || y >= self.height {
            return Err(FrameError::InvalidPixel);
        }

        let index = (y as usize * self.width as usize + x as usize) * self.format.bytes_per_pixel();

        if index + self.format.bytes_per_pixel() > self.data.len() {
            return Err(FrameError::InvalidPixel);
        }

        Ok(index)
    }

    /// It  changes the colour of a single pixel and returns the colour of that pixel
    pub fn replace_pixel(&mut self, position: &Pos, color: &Color) -> Result<Color, FrameError> {
        let index = self.pixel_index(position)?;
        let data = &mut self.data;
        let pixel = Color(data[index], data[index + 1], data[index + 2]);
        let Color(r, g, b) = *color;

        data[index] = r;
        data[index + 1] = g;
        data[index + 2] = b;
        Ok(pixel)
    }
    /// The get_pixel function returns the color of a pixel at a certain position
    pub fn get_pixel(&self, pos: &Pos) -> Result<Color, FrameError> {
        let index = self.pixel_index(pos)?;
        let data = self.data();
        Ok(Color(data[index], data[index + 1], data[index + 2]))
    }
}
