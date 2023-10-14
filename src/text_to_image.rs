use bevy::{
    self,
    prelude::Image,
    render::render_resource::{Extent3d, TextureDimension, TextureFormat},
};
use image::{ImageBuffer, Rgba};
use imageproc::drawing::draw_text_mut;
use rusttype::{Font, Scale};

pub fn text_to_image(text: &str) -> Image {
    let height = 100;
    let width = 200;
    let mut image_buffer = ImageBuffer::new(width, height);

    // fill with red
    for pixel in image_buffer.pixels_mut() {
        *pixel = Rgba([255u8, 0u8, 0u8, 255u8]);
    }

    let font = Font::try_from_bytes(include_bytes!("../assets/Nunito-Regular.ttf")).unwrap();
    

    draw_text_mut(
        &mut image_buffer,
        Rgba([255u8, 255u8, 255u8, 255u8]),
        55,
        21,
        Scale { x: 60.0, y: 60.0 },
        &font,
        text,
    );

    // flip y
    image_buffer = image::imageops::flip_vertical(&image_buffer);

    let bytes = image_buffer.into_raw();

    let image = Image::new_fill(
        Extent3d {
            width,
            height,
            depth_or_array_layers: 1,
        },
        TextureDimension::D2,
        &bytes,
        TextureFormat::Rgba8UnormSrgb,
    );

    return image;
}
