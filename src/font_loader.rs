use std::io::Cursor;

use glium::texture::RawImage2d;
extern crate image;

pub struct SpriteSheet {
    width: u8,
    height: u8,
    border: u8,
    start_x: u8,
    start_y: u8,
    rows: u8,
    columns: u8,
}

impl SpriteSheet {
    fn nth(&self, number: u8) -> (u8, u8, u8, u8) {
        return (0, 0, 0, 0)
    }
}

pub fn load_font(font_file: &str) -> RawImage2d<u8> {
    let image = image::load(
        Cursor::new(&include_bytes!("./profont.png")),
        image::ImageFormat::Png,
    )
    .unwrap()
    .to_rgba8();

    let image_dimensions = image.dimensions();
    let image = glium::texture::RawImage2d::from_raw_rgba(image.into_raw(), image_dimensions);
    image
} 
