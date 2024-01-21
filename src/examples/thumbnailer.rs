use std::io;
use std::io::Write;
use std::{fs, path::Path};
use rayon::prelude::*;

use std::time::{Instant};

pub fn thumbnailer() {
    print!("Enter the input directory containing images: ");
    io::stdout().flush().unwrap(); // Ensure the prompt is displayed

    let mut input_dir = String::new();
    io::stdin().read_line(&mut input_dir).expect("Failed to read line");
    let trimmed_input_dir = input_dir.trim();

    println!("Input Directory: {}", trimmed_input_dir);

    let output_dir = "./output_thumbnails";
    let thumbnail_width = 256;

    fs::create_dir_all(output_dir).expect("Failed to create output directory");

    let paths: Vec<_> = fs::read_dir(trimmed_input_dir)
        .expect("Failed to read input directory")
        .filter_map(|entry| entry.ok().map(|e| e.path()))
        .collect();

    let start = Instant::now();

    paths.par_iter().for_each(|path| {
        if let Ok(img) = image::open(path) {
            let aspect_ratio = img.height() as f32 / img.width() as f32;
            let thumbnail_height = (thumbnail_width as f32 * aspect_ratio) as u32;

            let thumbnail = img.thumbnail(thumbnail_width, thumbnail_height);
            let output_path = Path::new(output_dir).join(path.file_name().unwrap());
            thumbnail.save(output_path).expect("Failed to save thumbnail");
        }
    });

    let duration = start.elapsed();

    println!("Thumbnail generation complete.");
    println!("Time taken for {} images: {:?}", paths.len(), duration)
}