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
  - [x] refactor vector to image class
  - [ ] refactor sampling trait (sample image methods) to image class `next up @Florian`
  - [ ] supersampling (stratified sampling) `next up @Florian`
  - [ ] implement recursion depth `next up @Florian`
- [x] implement base functionalities for 2D image generation
  - [x] gradient color image
  - [x] disc in center
  - [x] polka dots with gradient
- [ ] implement base functionalities for 3D image generation
- [ ] implement different materials
- [ ] texture support
- [ ] implement bounding boxes checks of objects preventing unnecessary calculations
- [ ] implement multithreading support
