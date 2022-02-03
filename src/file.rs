use image;
use crate::bar_code::Color;

pub fn build_image(code: &Vec<Color>) -> image::RgbImage {
    let height = 50;
    let width = code.len() as u32;

    image::ImageBuffer::from_fn(width, height, |x, _| {
        match code[x as usize] {
            Color::Black => image::Rgb([0, 0, 0]),
            Color::White | Color::Space => image::Rgb([255, 255, 255]),
        }
    })
}
