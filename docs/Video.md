# Video

The `Video` type provides an interface for decoding and processing video streams in Drive.
It is responsible for extracting frames, handling timestamps, and interfacing with the decoding backend.

## Overview

The `Video` type wraps a media container and exposes frame-level access.
It uses FFmpeg as a backend for decoding and converts decoded frames into Drive's internal `Frame` representation.

## Structure

A `Video` instance contains:

| Field         | Description                                      |
| ------------- | ------------------------------------------------ |
| `ictx`        | Input format context                             |
| `decoder`     | Video decoder instance                           |
| `video_index` | Index of the selected video stream               |
| `time_base`   | Rational value representing the stream time base |

## Initialization

```rust
Video::open(path)
```

- Initializes FFmpeg
- Opens the media file
- Selects the best available video stream
- Creates a decoder from stream parameters

Returns an error if initialization fails.

## Frame Decoding

### Sequential Decoding

```rust
decode_next()
```

Decodes the next available frame. Returns a `VideoFrame` containing a `Frame` (RGBA format) and a timestamp.

Internally:
- Receives frames from the decoder
- Feeds packets when required
- Handles end-of-stream conditions

### Timestamp-Based Decoding

```rust
decode_frame(pts)
```

Seeks to the nearest keyframe at or before the target timestamp, then decodes forward until the desired frame is reached. Used for random access within a video stream.

## Frame Conversion

Decoded frames are converted into Drive's internal format:

- Source format → RGBA
- Uses FFmpeg scaling context
- Removes stride padding
- Converts interleaved data into planar format

The resulting `Frame` is compatible with all Drive processing operations.

## Timestamp Handling

Each decoded frame is associated with a timestamp:

| Field        | Description                      |
| ------------ | -------------------------------- |
| `value`      | Raw presentation timestamp (PTS) |
| `num`, `den` | Rational time base               |

This allows precise frame positioning.

## Audio Extraction

```rust
decode_audio()
```

Extracts audio from the video container, decodes audio frames using FFmpeg, and converts them into the `Track` abstraction.

Each audio frame contains:
- Timestamp
- Per-channel sample data

## Properties

```rust
width()       // Returns video width
height()      // Returns video height
time_base()   // Returns stream time base
```

---

## Video Encoding

The `VideoEncoder` type handles encoding processed frames.

### Initialization

```rust
VideoEncoder::open(path, width, height, time_base, frame_rate)
```

- Creates output context
- Configures encoder (H264)
- Sets resolution, pixel format, and timing
- Prepares scaling context (RGBA → YUV420)

### Encoding Frames

```rust
encode_frame(frame, index)
```

- Converts frame data into encoder format (YUV420)
- Assigns presentation timestamp
- Sends frame to encoder
- Writes encoded packets to output

### Finalization

```rust
finish()
```

- Flushes encoder
- Writes remaining packets
- Finalizes output file

### Internal Behavior

- Decoding is packet-driven
- Frames are processed sequentially
- Format conversion is explicit
- No implicit caching or buffering layer

---

## Error Handling

Errors are represented using `IOError`:

| Variant                | Description                        |
| ---------------------- | ---------------------------------- |
| `FileNotFound`         | File path does not exist           |
| `InvalidData`          | Malformed or unreadable media data |
| `EncodingFailed`       | Encoder encountered a failure      |
| `FFmpegError`          | General FFmpeg-level error         |
| `FFmpegDecodingFailed` | FFmpeg failed during decode        |
| `ReelError`            | Error from the Reel layer          |

---

## Current Capabilities

- Video file loading
- Frame-by-frame decoding
- Timestamp-based seeking
- Frame conversion to internal format
- Audio extraction from video
- Video encoding (H264)

## Limitations

- No streaming support
- No hardware acceleration
- Limited codec configuration
- No synchronization layer between audio and video

---

## Summary

The `Video` type provides controlled access to video data and integrates external decoding with Drive's internal processing pipeline.