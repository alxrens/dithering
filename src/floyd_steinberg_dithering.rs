use image::{DynamicImage, Luma, Rgba};





pub fn bw_dithering(img : &DynamicImage) -> DynamicImage {
    let mut img = img.to_luma8();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let old_pixel = img.get_pixel(x, y).0[0] as i32;
            let new_pixel  = if old_pixel > 64 { 255 } else { 0 };
            img.put_pixel(x, y, Luma([new_pixel as u8]));
            let quant_error = old_pixel - new_pixel as i32;

            if x + 1 < width {
                let mut pixel = img.get_pixel(x + 1, y).0[0] as i32;
                pixel += quant_error * 7 / 16;
                img.put_pixel( x + 1 , y, Luma([pixel.clamp(0, 255) as u8]));
            }

            if x > 0 && y +1 <height {
                let mut pixel = img.get_pixel(x-1, y + 1).0[0] as i32;
                pixel += quant_error * 3 / 16;
                img.put_pixel(x, y, Luma([pixel.clamp(0, 255) as u8]));
            }

            if y + 1 < height {
                let mut pixel = img.get_pixel(x, y + 1).0[0] as i32;
                pixel += quant_error * 5 / 16;
                img.put_pixel(x, y + 1 , Luma([pixel.clamp(0, 255) as u8]));
            }

            if x + 1 < width && y + 1 < height {
                let mut pixel = img.get_pixel(x + 1, y + 1).0[0] as i32;
                pixel += quant_error * 1 / 16;
                img.put_pixel(x + 1, y + 1, Luma([pixel.clamp(0, 255) as u8]));
            }
        }
    }

    DynamicImage::ImageLuma8(img.clone())
}



pub fn colored_dithering(img : &DynamicImage) -> DynamicImage {
    let mut img = img.to_rgba8();
    let (width, height) = img.dimensions();

    for y in 0..height {
        for x in 0..width {
            let old_pixel = img.get_pixel(x, y).clone();
            let new_pixel = Rgba([
                if old_pixel[0] > 64 { 255 } else { 0 },
                if old_pixel[1] > 64 { 255 } else { 0 },
                if old_pixel[2] > 64 { 255 } else { 0 },
                old_pixel[3]
            ]);

            img.put_pixel(x, y, new_pixel);
            let quant_error = [
                old_pixel[0] as i32 - new_pixel[0] as i32,
                old_pixel[1] as i32 - new_pixel[1] as i32,
                old_pixel[2] as i32 - new_pixel[2] as i32
            ];

            if x + 1 < width {
                for i in 0..3 {
                    let mut pixel = img.get_pixel(x + 1, y)[i] as i32;
                    pixel += quant_error[i] * 7 / 16;
                    img.get_pixel_mut(x + 1, y)[i] = pixel.clamp(0, 255) as u8;
                }
            }

            if x > 0 && y + 1 < height {
                for i in 0..3 {
                    let mut pixel = img.get_pixel(x - 1, y + 1)[i] as i32;
                    pixel += quant_error[i] * 3 / 16;
                    img.get_pixel_mut(x, y)[i] = pixel.clamp(0, 255) as u8;
                }
            }

            if y + 1 < height {
                for i in 0..3  {
                    let mut pixel = img.get_pixel(x, y + 1)[i] as i32;
                    pixel += quant_error[i] * 5 / 16;
                    img.get_pixel_mut(x, y + 1)[i] = pixel.clamp(0, 255) as u8;
                }
            }

            if x + 1 < width && y + 1 < height {
                for i in 0..3 {
                    let mut pixel = img.get_pixel(x + 1, y + 1)[i] as i32;
                    pixel += quant_error[i] * 1 / 16;
                    img.get_pixel_mut(x + 1, y + 1)[i] = pixel.clamp(0, 255) as u8;
                }
            }

        }
    }
    DynamicImage::ImageRgba8(img.clone())
}