use std::fs;
use std::path::Path;
use image::{GenericImageView, Rgba};

fn main() {

    let image_folder = "/Users/ilimturan/Desktop/white-images/orginal";

    let images = fs::read_dir(image_folder).expect("Error when reading image directory");

    for image in images {
        if let Ok(image) = image {
            let image_file_path = image.path();

            if let Some(extension) = image_file_path.extension() {
                if extension == "png" || extension == "jpg" || extension == "jpeg" {
                    println!("processing file");
                    process_image(&image_file_path);
                }
            }
        }
    }

    fn process_image(file_path: &Path) {
        let mut img = image::open(file_path).expect("Error when opening image");

        clear_white_areas(&mut img);

        let output_path = format!("{}_cleaned.png", file_path.file_stem().unwrap().to_str().unwrap());

        println!("new file name: {}", output_path);
        img.save(output_path).expect("Error when saving image");
    }

    fn clear_white_areas(img: &mut image::DynamicImage) {
        let mut rgba_img = img.to_rgba8();

        let (width, height) = rgba_img.dimensions();

        for x in 0..width {
            for y in 0..height {
                let pixel = rgba_img.get_pixel(x, y);
                if is_white_pixel(&pixel) {
                    let x_copy = x;
                    let y_copy = y;
                    rgba_img.put_pixel(x_copy, y_copy, Rgba([0, 0, 0, 0]));
                }
            }
        }

        *img = image::DynamicImage::ImageRgba8(rgba_img);
    }

    fn is_white_pixel(pixel: &image::Rgba<u8>) -> bool {
        const WHITE_THRESHOLD: u8 = 255;
        pixel[0] == WHITE_THRESHOLD && pixel[1] == WHITE_THRESHOLD && pixel[2] == WHITE_THRESHOLD
    }
}
