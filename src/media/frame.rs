use std::fmt;

// CONFIGS
#[derive(Debug, Clone, Copy)]
pub enum Color {
    RGB(u8, u8, u8),
    RGBA(u8, u8, u8, u8),
    Gray(u8),
}
impl Color {
    pub fn size(&self) -> usize {
        match *self {
            Self::RGB(..) => 3,
            Self::Gray(_) => 1,
            Self::RGBA(..) => 4,
        }
    }
}
#[derive(Debug, Clone, Copy)]
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
    pub fn ffmpeg_fmt(&self) -> &str {
        match *self {
            PixelFormat::RGB24 => "rgb24",
            PixelFormat::RGBA32 => "rgba",
            PixelFormat::Gray8 => "gray",
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

#[derive(Debug, Clone)]
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
    InvalidPixelFormat,
    BlitFailed,
    InvalidOpacityValue,
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
            FrameError::InvalidPixelFormat => {
                write!(f, "The PixelFormat for Frame and Color Does not match!")
            }
            FrameError::BlitFailed => {
                write!(f, "Overlapping Failed! Check Dimensions!")
            }
            FrameError::InvalidOpacityValue => {
                write!(f, "Opacity Value must be between 0 and 100")
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
    pub fn replace_pixel(&mut self, pos: &Pos, color: &Color) -> Result<Color, FrameError> {
        let pixel = self.get_pixel(pos)?;
        self.set_pixel(pos, color)?;
        Ok(pixel)
    }
    /// The get_pixel function returns the color of a pixel at a certain position
    pub fn get_pixel(&self, pos: &Pos) -> Result<Color, FrameError> {
        let index = self.pixel_index(pos)?;
        let format = self.format();
        let data = &self.data;

        let pixel = match format {
            PixelFormat::RGB24 => Color::RGB(data[index], data[index + 1], data[index + 2]),
            PixelFormat::RGBA32 => Color::RGBA(
                data[index],
                data[index + 1],
                data[index + 2],
                data[index + 3],
            ),
            PixelFormat::Gray8 => Color::Gray(data[index]),
        };

        Ok(pixel)
    }

    /// The set_pixel() method allows us to set the color of a pixel at a specific position
    pub fn set_pixel(&mut self, pos: &Pos, color: &Color) -> Result<(), FrameError> {
        let index = self.pixel_index(pos)?;
        let format = self.format();
        let data = &mut self.data;
        if format.bytes_per_pixel() != color.size() {
            return Err(FrameError::InvalidPixelFormat);
        }
        match *color {
            Color::RGB(r, g, b) => {
                data[index] = r;
                data[index + 1] = g;
                data[index + 2] = b;
            }
            Color::RGBA(r, g, b, a) => {
                data[index] = r;
                data[index + 1] = g;
                data[index + 2] = b;
                data[index + 3] = a;
            }

            Color::Gray(a) => {
                data[index] = a;
            }
        };

        Ok(())
    }
    pub fn set_alpha(&mut self, value: u8) -> Result<(), FrameError> {
        if self.format() != PixelFormat::RGBA32 {
            return Err(FrameError::InvalidPixelFormat);
        }
        let data = &mut self.data;
        for i in (3..data.len()).step_by(4) {
            data[i] = value;
        }
        Ok(())
    }
    pub fn opacity(&mut self, value: u8) -> Result<(), FrameError> {
        if self.format() != PixelFormat::RGBA32 {
            return Err(FrameError::InvalidPixelFormat);
        }
        if value > 100 {
            return Err(FrameError::InvalidOpacityValue);
        }
        let data = &mut self.data;

        for i in (3..data.len()).step_by(4) {
            data[i] = ((data[i] as u16 * value as u16) / 100) as u8;
        }
        Ok(())
    }
    //pub fn blit(&self, frame: &Frame) -> Result<Frame, FrameError> {}
}
