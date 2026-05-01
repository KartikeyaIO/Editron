# The Track Type

The `Track` type represents decoded audio data in Editron.

It provides a structured abstraction over audio samples and supports basic operations such as gain, mixing, normalization, and format conversion.

---

## Structure

A `Track` consists of:

- `sample_rate: u32`  
  Number of samples per second (e.g., 44100, 48000)

- `channels: u16`  
  Number of audio channels (e.g., 1 for mono, 2 for stereo)

- `buffer: Vec<AudioFrame>`  
  A sequence of audio frames, each containing per-channel sample data

---

## AudioFrame

Each `AudioFrame` contains:

- `time: TimeStamp`  
  Timestamp associated with the frame

- `data: Vec<Vec<f32>>`  
  Sample data organized as:
  
  data[channel_index][sample_index]

Samples are stored as normalized floating-point values in the range `[-1.0, 1.0]`.

---

## Design

- Audio is stored as `f32` normalized PCM
- Processing is performed in floating-point domain
- Channel data is stored per frame, not interleaved
- Conversion to interleaved formats is handled when needed

This design allows:

- Precise control over audio transformations
- Easy per-channel manipulation
- Separation between processing and export formats

---

## Accessors

- `sample_rate()` → returns sample rate  
- `channels()` → returns number of channels  
- `buffer()` → returns reference to audio frames  

---

## Core Operations

### Gain
`gain(db)`

- Applies gain in decibels to all samples
- Conversion:
`factor = 10^(dB / 20)`

Each sample is multiplied by this factor.

---

### Mixing
`Track::mix(a, b)`

- Mixes two tracks sample-by-sample

Requirements:

- Same sample rate  
- Same number of channels  
- Same number of frames  

Returns an error if tracks are incompatible.

---

### Normalization
`normalize()`

- Scales all samples so the maximum amplitude becomes 1.0
- Returns `false` if the track is empty or silent

---

## PCM Conversion

### To f32 PCM
`to_pcm_f32()`

- Flattens all frames into an interleaved stream
- Clamps values to `[-1.0, 1.0]`

---




## Internal Behavior

- Samples are processed per frame and per channel
- Interleaving is performed during conversion
- No implicit resampling or channel conversion is performed

---

## Error Handling

Errors are represented using `TrackError`:

- `MixingIsNotPossible`

Returned when track properties do not match for mixing.

---

## Current Capabilities

- Structured audio storage using frames
- Gain adjustment
- Track mixing
- Normalization
- Conversion to PCM formats

---

## Limitations

- No resampling support
- No time stretching or pitch shifting
- No streaming or lazy decoding
- No hardware acceleration

---

## Summary

The `Track` type provides:

- A structured representation of audio data
- Floating-point processing for precision
- Basic operations for audio manipulation

It serves as the core abstraction for audio processing in Editron.
