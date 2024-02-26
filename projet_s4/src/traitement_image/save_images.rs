use image::DynamicImage;

pub fn save_images(images: &Vec<DynamicImage>, output_dir: &str) -> Result<(), Box<dyn std::error::Error>> {
    // Create the output directory if it doesn't exist
    std::fs::create_dir_all(output_dir)?;

    // Iterate over the images vector
    for (idx, image) in images.iter().enumerate() {
        // Construct the file path
        let file_path = format!("{}/image_{}.png", output_dir, idx);

        // Save the image to the file path
        image.save(&file_path)?;
    }

    Ok(())
}
