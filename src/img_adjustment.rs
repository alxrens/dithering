use image::{imageops::contrast, DynamicImage};




pub fn adjust_contrast(img : &DynamicImage, contrast_factor : f32) -> DynamicImage {
    let mut img = img.to_rgba8();
    contrast(&mut img, contrast_factor);
    DynamicImage::ImageRgba8(img)
}