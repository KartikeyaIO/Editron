## The Track Type

`Track` is the core audio abstraction in Editron.  
It represents a decoded PCM audio stream stored as normalized floating-point samples.

### Structure

```rust
#[derive(Debug, Clone)]
pub struct Track {
    sample_rate: u32,
    channels: u16,
    buffer: Vec<f32>,
}
```

### Fields

- **sample_rate (`u32`)**  
  Number of samples per second (e.g., 44100, 48000).

- **channels (`u16`)**  
  Number of audio channels (e.g., 1 = mono, 2 = stereo).

- **buffer (`Vec<f32>`)**  
  Interleaved PCM samples stored as normalized floating-point values in the range `[-1.0, 1.0]`.

---

## Design Philosophy

- Audio is internally stored as **`f32` normalized PCM**
- Processing is performed in floating-point domain
- Clamping is applied only during export to integer formats
- `Track` acts as a safe, structured container for audio data

---

## 🔧 Accessors

```rust
pub fn sample_rate(&self) -> &u32
pub fn channels(&self) -> &u16
pub fn buffer(&self) -> &Vec<f32>
```

Provides read-only access to internal metadata and sample buffer.

---

## Core Operations

### Gain

```rust
pub fn gain(&mut self, db: f32)
```

Applies gain in decibels.

The conversion formula:

```
factor = 10^(dB / 20)
```

Each sample is multiplied by this factor.

---

### Mix

```rust
pub fn mix(track1: &Track, track2: &Track) -> Result<Track, TrackError>
```

Mixes two tracks by summing corresponding samples.

Requirements:
- Same sample rate
- Same number of channels
- Same buffer length

Returns `TrackError::MixingIsNotPossible` if constraints are not satisfied.

---

### PCM Conversion

#### To f32 PCM (Clamped)

```rust
pub fn to_pcm_f32(track: &Track) -> Vec<f32>
```

Returns samples clamped to `[-1.0, 1.0]`.

---

#### To i16 PCM

```rust
pub fn to_pcm_i16(track: &Track) -> Vec<i16>
```

Converts normalized floating-point samples to 16-bit integer PCM.

Formula:

```
sample_i16 = clamp(sample_f32, -1.0, 1.0) * i16::MAX
```

---

## Errors

```rust
pub enum TrackError {
    InvalidTrack,
    MixingIsNotPossible,
}
```

Represents possible failures in track operations.
