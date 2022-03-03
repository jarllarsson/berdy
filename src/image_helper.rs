use bevy::prelude::*;

pub fn get_pixel(
    pixel_pos: &UVec2,
    image: &Image
) -> Color {
    let pixel_idx : usize = ((pixel_pos.x + pixel_pos.y * image.texture_descriptor.size.width) * 4).try_into().unwrap();
    for _pixel in image.data.get(pixel_idx..pixel_idx+4) { // RGBA
        return Color::rgba_u8(_pixel[0], _pixel[1], _pixel[2], _pixel[3]);
    }
    return Color::BLACK;
}

pub fn set_pixel(
    pixel_pos: &UVec2,
    color: &Color,
    image: &mut Image
) {
    let pixel_idx : usize = ((pixel_pos.x + pixel_pos.y * image.texture_descriptor.size.width) * 4).try_into().unwrap();
    if let Some(pixel) = image.data.get_mut(pixel_idx..pixel_idx+4) { // RGBA
        pixel[0] = (color.r() * (u8::MAX as f32)) as u8;
        pixel[1] = (color.g() * (u8::MAX as f32)) as u8;
        pixel[2] = (color.b() * (u8::MAX as f32)) as u8;
        pixel[3] = (color.a() * (u8::MAX as f32)) as u8;
    }
}
