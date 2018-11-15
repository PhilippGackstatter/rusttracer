extern crate png;

// For reading and opening files
use std::path::Path;
use std::fs::File;
use std::io::BufWriter;
// To use encoder.set()
use self::png::HasParameters;

pub fn write_png_img(rgba_sequence: &Vec<u8>, width: u32, height: u32, path: String) {

    let path = Path::new(&path);
    let file = File::create(path).unwrap();
    let ref mut w = BufWriter::new(file);

    let mut encoder = png::Encoder::new(w, width, height); // Width is 2 pixels and height is 1.
    encoder.set(png::ColorType::RGBA).set(png::BitDepth::Eight);
    let mut writer = encoder.write_header().unwrap();

    writer.write_image_data(&rgba_sequence).unwrap(); // Save
}