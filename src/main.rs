use image::DynamicImage;
use image::imageops::{resize, FilterType};
use std::fs;
use std::time::Instant;
use rayon::prelude::*;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let folder_path: &String = &args[1];
    let output_path: &String = &args[2];
    let files: fs::ReadDir = fs::read_dir(folder_path).unwrap();

    println!("Reading images from: {}", folder_path);
    let now: Instant = Instant::now();
    // Read JPEG files and resize in parallel
    let images: Vec<DynamicImage> = files
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_par_iter()
        .map(|file| {
            let file_path = file.path();
            let img = image::open(&file_path).unwrap();
            let resized_img = resize(&img, 500, 500, FilterType::Lanczos3);
            // convert back to DynamicImage
            let resized_img = DynamicImage::ImageRgba8(resized_img);
            resized_img
        })
        .collect();
    let elapsed = now.elapsed();
    println!("Elapsed time for reading in {} images: {:.2?}", images.len(), elapsed);

    let output_file_path = format!("{}/output.gif", output_path);
    let mut output_file = fs::File::create(output_file_path).unwrap();
    let mut encoder = gif::Encoder::new(&mut output_file, images[0].width() as u16, images[0].height() as u16, &[]).unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();

    println!("Creating gif");
    let now = Instant::now();
    let mut i = 0;
    for img in &images {
        let mut palette = img.as_bytes().to_vec();
        println!("Image {}: {} {} {}", i, img.width(), img.height(), palette.len());
        let frame = gif::Frame::from_rgba_speed(img.width() as u16, img.height() as u16, palette.as_mut_slice(), 5);
        encoder.write_frame(&frame).unwrap();
        i += 1;
    }
    let elapsed = now.elapsed();
    println!("Elapsed time for creating gif: {:.2?}", elapsed);
}