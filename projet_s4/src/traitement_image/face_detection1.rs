extern crate image;
extern crate ndarray;

use image::{DynamicImage, GenericImageView};
use ndarray::{Array, Array2, Axis};
use ndarray::s;
// Convert DynamicImage to grayscale Array2
pub fn dynamic_image_to_gray_array(image: &DynamicImage) -> Array2<f64> {
    let (width, height) = image.dimensions();
    let grayscale_image = image.grayscale();
    let mut array = Array::zeros((width as usize, height as usize));

    for (x, y, pixel) in grayscale_image.pixels() {
        array[[x as usize, y as usize]] = pixel[0] as f64; // Assuming 1 channel (grayscale)
    }

    array
}

// Function to calculate the covariance matrix from multiple images
pub fn calculate_covariance_matrix(images: &Vec<DynamicImage>) -> Array2<f64> {
    let mut data = Vec::new();

    for image in images {
        let array = dynamic_image_to_gray_array(image);
        data.push(array);
    }

    let num_samples = data.len();
    let num_features = data[0].len_of(Axis(0));

    let mut concatenated_data = Array::zeros((num_samples, num_features));

    for (i, array) in data.iter().enumerate() {
        concatenated_data.slice_mut(s![i, ..]).assign(array);
    }

    let mean = concatenated_data.mean_axis(Axis(0)).unwrap();
    let centered = concatenated_data - &mean;

    let covariance = centered.t().dot(&centered) / (num_samples as f64);

    covariance
}

// Function to calculate eigenvectors and eigenvalues from the covariance matrix
