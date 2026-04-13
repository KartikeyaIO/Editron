# Frame Operations
## Noise Reduction Techniques:
- Median Filter or Gaussian Filter can be used
- Median filter prediction works by finding the median value of the neighbourhood pixels and replacing the original central pixel with that median value.
### Technical Challenges:
- kernel size : The larger the kernel size the more noise it removes but it can also remove fine details.
- High Computational Cost (use Parallel Processing)
### Read about: 
- Huang's Method: Histogram Based algorithms
- perreault and hebert -> Constant time Median fiter.
- Deblurring Techniques (weiner filter, Inverse Filter)
- Inpaiting.
- Denoising
##  Feature detection
- Corner Detection
- Blob Detection(LoG,DoG)
- Edge Linking
- Contour Detection
- Hough Transform (lines & circles)
## Morphological Operations
- Dilation
- Erosion
- Opening/Closing
- Skeletonization
- Hit or miss Transform
## Color Grading & Correction:
- White Balance
- Tone Curves
- Color Wheels
- Histogram Equilization
- Contrast Stretching
- Split Toning
## Temporal Operations
- Interpolation
- Differencing
- Motion Estimation
- Trim, split
- Rate Conversion
- Stabilization
## Blending 
- Alpha Blending -> Already Implemented
- layer Compositing
- Add, multiply, Screen overlay, etc.
- Masking (Manual or Rule Based)
- Keying(chroma and luma)
## geometric Transformations
- resize (Nearest. Bilinear, or Bicubic)
- Crop
- Rotate
- Flip/ Mirror
- Perspective Transform
- Affine Transform 
- Warping/ Distortions
## Convulation Filters
## Basic Operations:
- Brightness
- Contrast
- Saturation
- Color Space Conversion
- Thresholding
- Posterization
- Negative/ Inverison
- LUT - Lookup Table 
  