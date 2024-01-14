# Rust Raytracer

## General overview
> Implementation of a CPU Raytracer

### Authors
| Name           | Github  |
|----------------|---------|
| **Florian K.** | fnyrawr |
| **Max F.**     | FalkMaximilian    |

## ToDo
- [x] implement base image generation functionalities
  - [x] vector structure for data: [Px1.R Px1.G Px1.B Px2.R...]
  - [x] create constant color image (later refactor to basic Sampler trait)
  - [x] save image to PNG
  - [x] set image width and height (flexible options)
  - [x] implement gamma correction
  - [x] refactor sampling trait (sample image methods) to image class
  - [x] supersampling (stratified sampling)
- [x] implement base functionalities for 2D image generation
  - [x] gradient color image
  - [x] disc in center
  - [x] polka dots with gradient
- [ ] implement base functionalities for 3D image generation
  - [x] basic ray casting
  - [x] movable camera obscura
  - [x] implement recursion depth
  - [ ] implement different shapes
  - [ ] implement different materials
  - [ ] texture support
  - [ ] implement bounding boxes checks of objects preventing unnecessary calculations
- [ ] Implement optimizations
  - [x] implement faster random number generator
  - [ ] implement multithreading support
