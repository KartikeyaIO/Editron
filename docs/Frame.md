# The Frame Type

The `Frame` type represents a single decoded image in memory.  
It is the fundamental unit for pixel-level manipulation in Editron.

---

## Structure

A `Frame` consists of the following fields:

1. `width: u32`  
   Width of the image in pixels.

2. `height: u32`  
   Height of the image in pixels.

3. `format: PixelFormat`  
   Specifies the pixel encoding format (e.g., RGB24,RGBA32,Gray8).

4. `data: Vec<u8>`  
   Raw pixel buffer stored in row-major order.

The internal buffer is guaranteed to satisfy the invariant:

``` data.len() == width*height*bytes_per_pixel```


This constraint is enforced during construction to ensure memory safety and structural correctness.

---

## Implemented Methods

### Accessors

- `width()` → Returns the width of the frame.
- `height()` → Returns the height of the frame.
- `format()` → Returns the pixel format.
- `data()` → Returns a reference to the internal pixel buffer.

### Pixel Manipulation

- `brightness(delta: i32)`  
  Adjusts pixel intensity by adding `delta` to each channel value.  
  Values are clamped to remain within the valid `u8` range `[0, 255]`.
- Several other Functions for Pixel manipulation such as `set_pixel()`, `get_pixel()`, `replace_pixel()` were also added.
---

## Current Status

- Frame structure implemented.
- All Pixelformats in  `PixelFormat` enum are now officially supported.
- Brightness manipulation implemented.
- Buffer invariants enforced internally.
- Convolution Based Gaussian Blur is implemented.

---

## Planned Improvements


- Layer blending operations.
- Performance optimizations for large frames.
