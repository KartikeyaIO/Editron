# The Frame Type

The `Frame` type is the core abstraction used for representing and manipulating image data in Drive.

It provides a structured, format-aware representation of pixel data and exposes operations for direct pixel-level control.

---

## Structure

A `Frame` consists of:

- `width: u32`  
  Width of the frame in pixels

- `height: u32`  
  Height of the frame in pixels

- `data: PixelData`  
  Pixel buffer stored in a structured format

The following invariant is enforced at construction:

data.len() == width * height

This ensures consistency between dimensions and underlying storage.

---

## Pixel Representation

Pixel data is stored using the `PixelData` enum:

enum PixelData {
    RGB(Vec<u8>, Vec<u8>, Vec<u8>),
    RGBA(Vec<u8>, Vec<u8>, Vec<u8>, Vec<u8>),
    GRAY(Vec<u8>),
    YUV420(Vec<u8>, Vec<u8>, Vec<u8>),
}

### Design Notes

- RGB and RGBA are stored as separate channel buffers (planar format)
- GRAY stores a single luminance channel
- YUV420 uses subsampled chroma planes (4:2:0)

This design avoids implicit assumptions about memory layout and allows explicit control over conversions.

---

## Construction

Frame::new(width, height, data)

- Validates that the provided buffer matches expected dimensions
- Returns an error if the buffer size is inconsistent

---

## Accessors

- width() → returns frame width  
- height() → returns frame height  
- data() → returns immutable reference to pixel data  
- data_mut() → returns mutable reference to pixel data  

---

## Pixel Operations

### Indexing

All pixel operations are position-based:

Pos(x, y)

Bounds checking is enforced internally.

---

### Read / Write

- get_pixel(pos) → returns pixel color  
- set_pixel(pos, color) → sets pixel color  
- replace_pixel(pos, color) → replaces and returns previous value  

---

### Brightness

brightness(pos, delta)

- Applies an additive change to pixel intensity  
- Clamped to [0, 255]  
- Works across RGB, RGBA, GRAY, and YUV (luma only)

---

### Contrast

contrast(pos, factor)

- Applies contrast adjustment around midpoint (128)  
- Supports RGB, RGBA, GRAY, and YUV (luma)

---

### Opacity and Alpha

- set_alpha(pos, value) → sets alpha channel  
- opacity(pos, value) → scales alpha percentage  

Only valid for RGBA frames.

---

### Blending

blend(other_frame, alpha)

- Combines two frames using linear interpolation  
- Requires:
  - Same dimensions
  - Same pixel format  

Returns a new blended frame.

---

## Format Conversion

### RGBA Conversion

to_rgba8(width, height)

- Converts internal pixel format to RGBA  
- Supports RGB, GRAY, and YUV420  
- YUV420 conversion uses integer-based color transformation  

---

### Interleaving

interleave()

- Converts planar pixel data into interleaved format  
- Useful for encoding and external libraries  

---

## Internal Utilities

### Pixel Indexing

pixel_index(pos)

- Converts (x, y) into linear buffer index  
- Validates bounds before access  

---

### Padding

Frames can be padded to match dimensions for operations like blending.

- Centers original frame inside a larger buffer  
- Preserves pixel data while expanding dimensions  

---

## Error Handling

Errors are represented using `FrameError`:

- InvalidFrameSize  
- InvalidPixel  
- InvalidPixelFormat  
- InvalidOpacityValue  
- BlendingFailed  
- EmptyFrame  
- YUVNotApplied  

All operations validate format and bounds before execution.

---

## Current Capabilities

- Structured pixel storage (RGB, RGBA, GRAY, YUV420)
- Pixel-level manipulation
- Brightness and contrast adjustment
- Alpha and opacity control
- Frame blending
- Format conversion to RGBA
- Interleaving for encoding

---

## Limitations

- Many operations are pixel-wise and not vectorized
- YUV support is limited in certain operations
- No hardware acceleration
- No parallel processing

---

## Summary

The `Frame` type provides:

- Explicit control over pixel data
- Format-aware operations
- A consistent interface for image processing

It serves as the foundation for all visual processing in Drive.