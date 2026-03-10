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
    pub fn r(&self) -> u8 {
        match self {
            Self::RGB(r, _, _) => *r,
            Self::RGBA(r, _, _, _) => *r,
            _ => 0,
        }
    }
    pub fn g(&self) -> u8 {
        match self {
            Self::RGB(_, g, _) => *g,
            Self::RGBA(_, g, _, _) => *g,
            _ => 0,
        }
    }
    pub fn b(&self) -> u8 {
        match self {
            Self::RGB(_, _, b) => *b,
            Self::RGBA(_, _, b, _) => *b,
            _ => 0,
        }
    }
}
#[derive(Debug, Clone, Copy)]
pub struct Pos(pub u32, pub u32);

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord)]
/// PixelData store the actual Data of a Frame
pub enum PixelData {
    RGB(Vec<u8>, Vec<u8>, Vec<u8>),
    RGBA(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>),
    GRAY(Vec<u8>),
}

impl PixelData {
    pub fn bytes_per_pixel(&self) -> usize {
        match self {
            PixelData::RGB(..) => 3,
            PixelData::GRAY(_) => 1,
            PixelData::RGBA(..) => 4,
        }
    }
    pub fn ffmpeg_fmt(&self) -> &str {
        match *self {
            PixelData::RGB(..) => "rgb24",
            PixelData::GRAY(_) => "gray",
            PixelData::RGBA(..) => "rgba",
        }
    }
    pub fn len(&self) -> usize {
        match self {
            PixelData::RGB(d, _, _) => d.len(),
            PixelData::GRAY(d) => d.len(),
            PixelData::RGBA(d, _, _, _) => d.len(),
        }
    }
}

#[derive(Debug, Clone)]
/// Frame Type is used to store an image or a single frame from a video.
pub struct Frame {
    width: u32,
    height: u32,

    data: PixelData,
}

#[derive(Debug)]
pub enum FrameError {
    InvalidFrameSize,
    InvalidPixel,
    InvalidPixelFormat,
    BlitFailed,
    InvalidOpacityValue,
    EmptyFrame,
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
            FrameError::EmptyFrame => {
                write!(
                    f,
                    "The data in Frame is Empty! The Image might be corrupted."
                )
            }
        }
    }
}

impl std::error::Error for FrameError {}

impl Frame {
    pub fn new(width: u32, height: u32, data: PixelData) -> Result<Self, FrameError> {
        let expected_len = width as usize * height as usize;

        if data.len() == expected_len {
            Ok(Self {
                width,
                height,
                data,
            })
        } else {
            Err(FrameError::InvalidFrameSize)
        }
    }
    /// The brightness function is used to adjust the brightness of a Frame
    pub fn brightness(&mut self, delta: i16) {
        let clamp = |v: u8| (v as i16 + delta).clamp(0, 255) as u8;

        match &mut self.data {
            PixelData::RGB(r, g, b) | PixelData::RGBA(r, g, b, _) => {
                r.iter_mut().for_each(|v| *v = clamp(*v));
                g.iter_mut().for_each(|v| *v = clamp(*v));
                b.iter_mut().for_each(|v| *v = clamp(*v));
            }
            PixelData::GRAY(l) => l.iter_mut().for_each(|v| *v = clamp(*v)),
        }
    }

    pub fn width(&self) -> u32 {
        self.width
    }
    pub fn height(&self) -> u32 {
        self.height
    }
    pub fn data(&self) -> &PixelData {
        &self.data
    }
    pub fn data_mut(&mut self) -> &mut PixelData {
        &mut self.data
    }
}
impl Frame {
    pub fn pixel_index(&self, pos: &Pos) -> Result<usize, FrameError> {
        let Pos(x, y) = *pos;
        if x >= self.width || y >= self.height {
            return Err(FrameError::InvalidPixel);
        }
        Ok(y as usize * self.width as usize + x as usize)
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
        let data = self.data();

        match data {
            PixelData::RGB(r, g, b) => Ok(Color::RGB(r[index], g[index], b[index])),
            PixelData::RGBA(r, g, b, a) => Ok(Color::RGBA(r[index], g[index], b[index], a[index])),
            PixelData::GRAY(l) => Ok(Color::Gray(l[index])),
        }
    }

    /// The set_pixel() method allows us to set the color of a pixel at a specific position
    pub fn set_pixel(&mut self, pos: &Pos, color: &Color) -> Result<(), FrameError> {
        let index = self.pixel_index(pos)?;

        if self.data.bytes_per_pixel() != color.size() {
            return Err(FrameError::InvalidPixelFormat);
        }

        match (&mut self.data, color) {
            (PixelData::RGB(r, g, b), Color::RGB(rv, gv, bv)) => {
                r[index] = *rv;
                g[index] = *gv;
                b[index] = *bv;
            }
            (PixelData::RGBA(r, g, b, a), Color::RGBA(rv, gv, bv, av)) => {
                r[index] = *rv;
                g[index] = *gv;
                b[index] = *bv;
                a[index] = *av;
            }
            (PixelData::GRAY(l), Color::Gray(v)) => {
                l[index] = *v;
            }
            _ => return Err(FrameError::InvalidPixelFormat),
        }

        Ok(())
    }
    pub fn set_alpha(&mut self, value: u8) -> Result<(), FrameError> {
        if self.data.bytes_per_pixel() != 4 {
            return Err(FrameError::InvalidPixelFormat);
        }
        let data = &mut self.data;
        match data {
            PixelData::RGBA(_, _, _, a) => a.fill(value),
            _ => return Err(FrameError::InvalidPixelFormat),
        }
        Ok(())
    }
    pub fn opacity(&mut self, value: u8) -> Result<(), FrameError> {
        if self.data.bytes_per_pixel() != 4 {
            return Err(FrameError::InvalidPixelFormat);
        }
        if value > 100 {
            return Err(FrameError::InvalidOpacityValue);
        }
        let data = &mut self.data;

        match data {
            PixelData::RGBA(_, _, _, a) => {
                for i in a {
                    *i = ((*i as u16 * value as u16) / 100) as u8;
                }
            }
            _ => return Err(FrameError::InvalidPixelFormat),
        }
        Ok(())
    }
    pub fn contrast(&mut self) -> Result<(), FrameError> {
        let data = &mut self.data;
        match data {
            PixelData::GRAY(l) => {
                let max = *l.iter().max().ok_or(FrameError::EmptyFrame)?;
                let min = *l.iter().min().ok_or(FrameError::EmptyFrame)?;
                let offset = max - min;
                if offset == 0 {
                    return Ok(());
                }
                l.iter_mut()
                    .for_each(|v| *v = ((*v - min) as u16 * 255 / offset as u16) as u8);
            }
            PixelData::RGB(r, g, b) | PixelData::RGBA(r, g, b, _) => {
                for channel in [r, g, b] {
                    let max = *channel.iter().max().ok_or(FrameError::EmptyFrame)?;
                    let min = *channel.iter().min().ok_or(FrameError::EmptyFrame)?;
                    let offset = max - min;
                    if offset == 0 {
                        continue;
                    }
                    channel
                        .iter_mut()
                        .for_each(|v| *v = ((*v - min) as u16 * 255 / offset as u16) as u8);
                }
            }
        }
        Ok(())
    }
}
