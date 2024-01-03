fn main() {
    println!("generating test image");

    // image dimensions
    const WIDTH: usize = 400;
    const HEIGHT: usize = 300;
    const DATA_SIZE: usize = 3*WIDTH*HEIGHT;

    // data array containing image's flattened pixel data (like [R G B R G B R G B ...])
    let mut data: [f32; DATA_SIZE] = [0.0; DATA_SIZE];

    // assign constant value
    for i in 0..data.len() {
        data[i] = 100.0;
    }
}
