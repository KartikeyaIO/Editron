use crate::media::frame::Frame;
use crate::media::frame::{FrameError, PixelData};
use ffmpeg::media::Type;
use ffmpeg::software::scaling::{context::Context, flag::Flags};
use ffmpeg::util::format::pixel::Pixel;
use ffmpeg_next as ffmpeg;
use ffmpeg_next::format::input;
use image::GenericImageView;
use image::codecs::png::PngEncoder;
use image::io::Reader;
use image::{ExtendedColorType, ImageEncoder};
use reel::{
    error::ReelResult,
    frame::{FrameHeader, YuvFrame},
    header::FileHeader,
    writer::ReelWriter,
};
use std::fs::File;
use std::io::BufWriter;
use std::time::Instant;

#[derive(Debug)]
pub enum IOError {
    FileNotFound,
    InvalidData,
    EncodingFailed,
    FFmpegError,
    FFmpegDecodingFailed,
    ReelError,
}

pub fn load_image(path: &str, fmt: &str) -> Result<Frame, FrameError> {
    let img = Reader::open(path)
        .map_err(|_| FrameError::EmptyFrame)?
        .decode()
        .map_err(|_| FrameError::EmptyFrame)?;

    let (width, height) = img.dimensions();
    let pixel_count = (width * height) as usize;

    let data = match fmt.to_lowercase().as_str() {
        "rgb" => {
            let rgb = img.to_rgb8();
            let raw = rgb.as_raw();

            let mut r = Vec::with_capacity(pixel_count);
            let mut g = Vec::with_capacity(pixel_count);
            let mut b = Vec::with_capacity(pixel_count);

            for chunk in raw.chunks_exact(3) {
                r.push(chunk[0]);
                g.push(chunk[1]);
                b.push(chunk[2]);
            }

            PixelData::RGB(r, g, b)
        }

        "rgba" => {
            let rgba = img.to_rgba8();
            let raw = rgba.as_raw();

            let mut r = Vec::with_capacity(pixel_count);
            let mut g = Vec::with_capacity(pixel_count);
            let mut b = Vec::with_capacity(pixel_count);
            let mut a = Vec::with_capacity(pixel_count);

            for chunk in raw.chunks_exact(4) {
                r.push(chunk[0]);
                g.push(chunk[1]);
                b.push(chunk[2]);
                a.push(chunk[3]);
            }

            PixelData::RGBA(r, g, b, a)
        }

        "gray" | "l8" => {
            let gray = img.to_luma8();
            PixelData::GRAY(gray.into_raw())
        }

        "yuv420" => {
            // Enforce even dimensions
            if width % 2 != 0 || height % 2 != 0 {
                return Err(FrameError::InvalidFrameSize);
            }

            let rgb = img.to_rgb8();
            let raw = rgb.as_raw();

            let mut y_plane = vec![0u8; (width * height) as usize];
            let mut u_plane = vec![0u8; ((width / 2) * (height / 2)) as usize];
            let mut v_plane = vec![0u8; ((width / 2) * (height / 2)) as usize];

            // --- Y PLANE ---
            for y in 0..height {
                for x in 0..width {
                    let idx = ((y * width + x) * 3) as usize;

                    let r = raw[idx] as f32;
                    let g = raw[idx + 1] as f32;
                    let b = raw[idx + 2] as f32;

                    let y_val = (0.299 * r + 0.587 * g + 0.114 * b).clamp(0.0, 255.0) as u8;

                    y_plane[(y * width + x) as usize] = y_val;
                }
            }

            // --- U & V (4:2:0 subsampling) ---
            for y in (0..height).step_by(2) {
                for x in (0..width).step_by(2) {
                    let mut u_sum = 0.0;
                    let mut v_sum = 0.0;

                    // 2x2 block
                    for dy in 0..2 {
                        for dx in 0..2 {
                            let px = x + dx;
                            let py = y + dy;

                            let idx = ((py * width + px) * 3) as usize;

                            let r = raw[idx] as f32;
                            let g = raw[idx + 1] as f32;
                            let b = raw[idx + 2] as f32;

                            u_sum += -0.169 * r - 0.331 * g + 0.5 * b + 128.0;
                            v_sum += 0.5 * r - 0.419 * g - 0.081 * b + 128.0;
                        }
                    }

                    let index = ((y / 2) * (width / 2) + (x / 2)) as usize;

                    u_plane[index] = (u_sum * 0.25).clamp(0.0, 255.0) as u8;
                    v_plane[index] = (v_sum * 0.25).clamp(0.0, 255.0) as u8;
                }
            }

            PixelData::YUV420(y_plane, u_plane, v_plane)
        }

        _ => return Err(FrameError::InvalidPixelFormat),
    };

    Frame::new(width, height, data)
}

pub fn encode_image(frame: &Frame, path: &str) -> Result<(), IOError> {
    let file = match File::create(path) {
        Ok(val) => val,
        Err(_) => return Err(IOError::FileNotFound),
    };
    let writer = BufWriter::new(file);
    let encoder = PngEncoder::new(writer);
    let width = frame.width();
    let height = frame.height();
    let data = match frame.data().to_rgba8(width, height) {
        Ok(val) => val,
        Err(_) => return Err(IOError::InvalidData),
    };

    encoder
        .write_image(
            &data.interleave()[..],
            width,
            height,
            ExtendedColorType::Rgba8,
        )
        .map_err(|_| IOError::EncodingFailed)
}

fn copy_plane(
    frame: &ffmpeg::util::frame::Video,
    plane: usize,
    width: usize,
    height: usize,
) -> Vec<u8> {
    let stride = frame.stride(plane);
    let data = frame.data(plane);
    let mut out = Vec::with_capacity(width * height);
    for row in 0..height {
        let start = row * stride;
        out.extend_from_slice(&data[start..start + width]);
    }
    out
}
// Video
pub fn convert_to_reel(input_path: &str, output_path: &str) -> Result<(), IOError> {
    ffmpeg::init().map_err(|_| IOError::FFmpegError)?;
    let mut ictx = match input(input_path) {
        Ok(val) => val,
        Err(_) => {
            return Err(IOError::FFmpegError);
        }
    };
    let input = match ictx.streams().best(Type::Video).ok_or("No video stream") {
        Ok(val) => val,
        Err(_) => {
            return Err(IOError::FFmpegError);
        }
    };
    let video_stream_index = input.index();
    let mut decoder = ffmpeg::codec::context::Context::from_parameters(input.parameters())
        .map_err(|_| IOError::FFmpegError)?
        .decoder()
        .video()
        .map_err(|_| IOError::FFmpegError)?;
    let mut scaler = match Context::get(
        decoder.format(),
        decoder.width(),
        decoder.height(),
        Pixel::YUV420P, // Destination format
        decoder.width(),
        decoder.height(),
        Flags::BILINEAR,
    ) {
        Ok(val) => val,
        Err(_) => return Err(IOError::FFmpegError),
    };
    let total_frames = input.frames() as u64;
    let frame_rate = input.avg_frame_rate(); // Rational
    let fps_num = frame_rate.numerator() as u32;
    let fps_den = frame_rate.denominator() as u32;
    let header = FileHeader::new(
        total_frames,
        decoder.width(),
        decoder.height(),
        fps_num,
        fps_den,
    );

    let mut writer = ReelWriter::new(output_path, header).map_err(|_| IOError::ReelError)?;
    let mut index = 0;
    for (stream, packet) in ictx.packets() {
        if stream.index() == video_stream_index {
            decoder
                .send_packet(&packet)
                .map_err(|_| IOError::FFmpegDecodingFailed)?;
            let mut decoded = ffmpeg::util::frame::Video::empty();
            while decoder.receive_frame(&mut decoded).is_ok() {
                //let t0 = Instant::now();
                let mut yuv_frame = ffmpeg::util::frame::Video::empty();
                scaler
                    .run(&decoded, &mut yuv_frame)
                    .map_err(|_| IOError::FFmpegDecodingFailed)?;
                //println!("scale: {:?}", t0.elapsed());
                println!("Y plane buffer size: {}", yuv_frame.data(0).len());
                println!(
                    "Y plane expected:    {}",
                    decoder.width() * decoder.height()
                );
                println!("Y stride:            {}", yuv_frame.stride(0));
                // Separating Channels
                //let t1 = Instant::now();
                let width = decoder.width() as usize;
                let height = decoder.height() as usize;

                let ydata = copy_plane(&yuv_frame, 0, width, height);
                let udata = copy_plane(&yuv_frame, 1, width / 2, height / 2);
                let vdata = copy_plane(&yuv_frame, 2, width / 2, height / 2);
                //println!("copy: {:?}", t1.elapsed());
                let frame_header = FrameHeader::new(
                    ydata.len() as u32,
                    udata.len() as u32,
                    vdata.len() as u32,
                    index,
                );
                let frame = YuvFrame::new(frame_header, &ydata[..], &udata[..], &vdata[..]);
                //let t2 = Instant::now();

                writer
                    .write_frame(frame)
                    .map_err(|_| IOError::EncodingFailed)?;
                index += 1;
                //println!("write: {:?}", t2.elapsed());
            }
        }
    }
    writer
        .finalize(fps_num, fps_den)
        .map_err(|_| IOError::EncodingFailed)?;
    Ok(())
}
