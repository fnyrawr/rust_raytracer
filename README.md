# Rust Raytracer

## General overview
> Implementation of a CPU Raytracer

### Authors
| Name           | Github  |
|----------------|---------|
| **Florian K.** | fnyrawr |
| **Max F.**     | FalkMaximilian    |

## ToDo
- [ ] implement base image generation functionalities `@Florian 03.01.2023`
  - [ ] Array structure for data: [Px1.R Px1.G Px1.B Px2.R...]
  - [ ] create constant color image (later refactor to basic Sampler trait)
  - [ ] save image to PNG
  - [ ] set image width and height (flexible options)
  - [ ] supersampling (stratified sampling)
- [ ] implement base functionalities for 2D image generation
  - [ ] gradient color image
  - [ ] disc in center
  - [ ] polka dots with gradient
- [ ] implement base functionalities for 3D image generation
- [ ] implement different materials
- [ ] texture support
- [ ] implement bounding boxes checks of objects preventing unnecessary calculations
- [ ] implement multithreading support
