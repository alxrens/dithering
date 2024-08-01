use image::open;
use std::path::Path;
use std::path::PathBuf;
use clap::{arg, value_parser, ArgAction, Command};

mod floyd_steinberg_dithering;
mod img_adjustment;

fn main() {
    let matches =Command::new("Dithering")
        .version("0.1")
        .about("it dithers your image lol")
        .arg(
            arg!(-i --image <FILE> "input the path to your image (required)")
            .required(true)
            .short('I')
            .long("image")
        )
        .arg(
            arg!(-o --output <FILE> "output the path to your image (required)")
            .required(true)
            .short('O')
            .long("output")
        )
        .arg(arg!(
            -c --contrast <FLOAT> "contrast factor"
        ).default_value("10.0")
        .value_parser(value_parser!(f32))
        .short('C')
        .long("contrast")
        )
        .arg(
            arg!(-l --colored <BOOL> "colored dithering")
            .default_value("false")
            .action(ArgAction::SetTrue)
            .short('L')
            .long("colored")
        )
        .arg(
            arg!(-t --threshold <INT> "threshold")
            .default_value("128")
            .value_parser(value_parser!(i32))
            .short('T')
            .long("threshold")
        )
        .get_matches();

    let img = match open(&Path::new(matches.get_one::<String>("image").expect("image is required"))) {
        Ok(i) => i,
        Err(why) => panic!("error : {}", why)
    };
    
    let amount_contrast = matches.get_one::<f32>("contrast").expect("contrast is required");
    let output_path = matches.get_one::<String>("output").expect("please input your desired path and filename for the output image");

    let img = img_adjustment::adjust_contrast(&img, amount_contrast);

    let output_path = Path::new(output_path);

    if output_path.is_dir() {
        panic!("output path is a directory");
    }
    if output_path.extension().is_none() {
        panic!("output path has no extension");
    }

    if *matches.get_one::<bool>("colored").expect("colored is required") {
        let treshold = *matches.get_one::<i32>("threshold").expect("threshold is required");

        if treshold > 255 || treshold < 0 {
            panic!("threshold must be between 0 and 255");
        }

        let dithered_image = floyd_steinberg_dithering::colored_dithering(&img, treshold as u8);
        let out_image = dithered_image.to_rgb8();
        out_image.save(PathBuf::from(output_path)).expect("Failed to save image");
    } else {
        let treshold = *matches.get_one::<i32>("threshold").expect("threshold is required");
        
        if treshold > 255 || treshold < 0 {
            panic!("threshold must be between 0 and 255");
        }

        let dithered_image = floyd_steinberg_dithering::bw_dithering(&img, treshold);
        dithered_image.save(PathBuf::from(output_path)).expect("Failed to save image");
    }

}
