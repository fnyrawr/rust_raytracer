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
  - [x] Array structure for data: [Px1.R Px1.G Px1.B Px2.R...] `base_image_generation`
  - [x] create constant color image (later refactor to basic Sampler trait) `base_image_generation`
  - [x] save image to PNG `base_image_generation`
  - [x] set image width and height (flexible options) `base_image_generation`
  - [ ] supersampling (stratified sampling) `(will follow later)`
- [x] implement base functionalities for 2D image generation
  - [x] gradient color image
  - [x] disc in center
  - [x] polka dots with gradient
- [ ] implement base functionalities for 3D image generation
- [ ] implement different materials
- [ ] texture support
- [ ] implement bounding boxes checks of objects preventing unnecessary calculations
- [ ] implement multithreading support
