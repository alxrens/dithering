use image::open;
use std::path::Path;



mod floyd_steinberg_dithering;
mod img_adjustment;

fn main() {

    let img_path = "cat.jpg";
    let img = open(&Path::new(img_path)).expect("Failed to open image");


    let img = img_adjustment::adjust_contrast(&img, 10.0);

    let dithered_image = floyd_steinberg_dithering::bw_dithering(&img);
    let col_dithered_image = floyd_steinberg_dithering::colored_dithering(&img);

    let final_col_img = col_dithered_image.to_rgb8();

    dithered_image.save("cat_dithered.jpg").expect("Failed to save image");
    // col_dithered_image.save("cat_col_dithered.jpg").expect("Failed to save image");
    final_col_img.save("cat_col_dithered.jpg").expect("Failed to save image");

}
