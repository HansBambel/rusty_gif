use image::ImageBuffer;
use image::imageops::{resize, FilterType};
use std::fs;
use std::time::Instant;
use rayon::prelude::*;
use std::io::BufWriter;
use indicatif::ProgressIterator;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let folder_path: &String = &args[1];
    let output_path: &String = &args[2];
    let files: fs::ReadDir = fs::read_dir(folder_path).unwrap();

    println!("Reading images from: {}", folder_path);
    let total = Instant::now();
    let new_width: u32 = 500;
    // Read JPEG files and resize in parallel
    let images: Vec<ImageBuffer<image::Rgba<u8>, Vec<u8>>> = files
        .collect::<Result<Vec<_>, _>>()
        .unwrap()
        .into_par_iter()
        .map(|file| {
            let file_path = file.path();
            let img = image::open(&file_path).unwrap().into_rgba8();
            // preserve aspect ratio in resized image
            let aspect_ratio = img.width() as f32 / img.height() as f32;
            let new_height = (new_width as f32 / aspect_ratio) as u32;
            let resized_img = resize(&img, new_width, new_height, FilterType::Lanczos3);
            resized_img
        })
        .collect();
    let elapsed_reading = total.elapsed();
    println!("Elapsed time for reading in {} images: {:.2?}", images.len(), elapsed_reading);

    let output_file_path: String = format!("{}/output.gif", output_path);
    let output_file: fs::File = fs::File::create(output_file_path).unwrap();
    let output: BufWriter<fs::File> = BufWriter::new(output_file);
    let mut encoder: gif::Encoder<BufWriter<fs::File>> = gif::Encoder::new(output, images[0].width() as u16, images[0].height() as u16, &[]).unwrap();
    encoder.set_repeat(gif::Repeat::Infinite).unwrap();

    println!("Creating gif");
    let now = Instant::now();
    for img in images.iter().progress(){
        let mut pixels = img.as_raw().to_vec();
        let frame = gif::Frame::from_rgba_speed(img.width() as u16, img.height() as u16, &mut pixels, 10);
        encoder.write_frame(&frame).unwrap();
    }
    let elapsed = now.elapsed();
    println!("Elapsed time for creating gif: {:.2?}", elapsed);
    println!("Total time: {:.2?}", total.elapsed());
}
