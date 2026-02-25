## Filter System

Filters in Editron are modular image-processing units that transform a `Frame`.

The system is built around a simple trait abstraction that allows new filters to be implemented consistently.

---

##  Filter Trait

```rust
pub trait Filter {
    fn apply(&self, frame: Frame) -> Frame {
        frame
    }
}
```

### Design Goals

- Provide a unified interface for all image filters
- Enable polymorphic behavior
- Allow filters to own internal configuration (e.g., kernels)
- Keep the processing pipeline extensible

Each filter takes ownership of a `Frame` and returns a transformed `Frame`.

---

## Example: Gaussian Blur

```rust
pub struct GaussianBlur {
    pub sigma: f32,
    kernel: Vec<f32>,
}
```

### Parameters

- **sigma (`f32`)**  
  Controls the spread of the Gaussian distribution.  
  Larger values produce stronger blur.

- **kernel (`Vec<f32>`)**  
  Precomputed 1D Gaussian kernel, normalized so that all weights sum to 1.

---

## Kernel Construction

```rust
pub fn build_kernel(sigma: &f32) -> Vec<f32>
```

Kernel radius is computed as:

```
radius = ceil(3 * sigma)
```

Each weight is calculated using the Gaussian function:

```
weight = exp( -x² / (2σ²) )
```

The kernel is then normalized:

```
k[i] = k[i] / sum(k)
```

This ensures brightness preservation after convolution.

---

## Blur Implementation Strategy

The blur is implemented using **separable convolution**:

1. Horizontal pass
2. Vertical pass

Instead of applying a 2D Gaussian kernel directly, the algorithm performs two 1D convolutions.

### Why Separable Convolution?

A 2D Gaussian kernel of size `N x N` has complexity:

```
O(N²)
```

Using separable convolution reduces complexity to:

```
O(2N)
```

This significantly improves performance for larger kernels.

---

## Algorithm Overview

For each pixel:

1. Iterate across kernel offsets
2. Clamp coordinates at image boundaries
3. Multiply neighboring pixel values by kernel weights
4. Accumulate weighted sums for R, G, B channels
5. Clamp final color values to `[0, 255]`
6. Write result back to frame

Edge handling is performed using coordinate clamping.

---

## Architectural Benefits

- Filters are independent modules
- Kernel generation is decoupled from application
- Performance optimized via separable convolution
- Easy to extend with new filters (Sharpen, Edge Detection, etc.)

---

## Summary

The filter system in Editron provides:

- A clean abstraction layer
- Extensible processing pipeline
- Efficient convolution-based image transformations
- Separation of configuration (`sigma`) and execution (`apply`)
