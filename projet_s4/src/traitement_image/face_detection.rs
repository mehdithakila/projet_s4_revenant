use image::*;
use super::convert_to_grey::*;
use ndarray::*;
use ndarray_linalg::*;
use ndarray_linalg::error::LinalgError;
pub fn conv_mat(images:Vec<DynamicImage>) //->Array2<f64>
{
    let mut data:Vec<Array2<f64>> = Vec::new();
    let (width,height)=images[0].dimensions();
    let l=images.len();
    for image in images
    {
        let grey_image=convert_to_grey(&image);
        let flattened_img: Vec<f64> = grey_image.pixels().map(|px| px.0 as f64).collect();

        let img_matrix = Array2::from_shape_vec((1, flattened_img.len()), flattened_img).unwrap();

        data.push(img_matrix);     
    }

    let data_matrix = stack(Axis(0), &data.iter().map(|arr| arr.view()).collect::<Vec<_>>()).unwrap();
    // Compute the mean vector
    let mean_vector = data_matrix.mean_axis(Axis(0)).unwrap();

    // Center the data by subtracting the mean vector
    let centered_data = data_matrix - &mean_vector;

    // Compute the covariance matrix
    let num_samples = data_matrix.shape()[0] as f64;
    let covariance_matrix = centered_data.into_shape((num_samples, width * height)).unwrap().t().dot(&centered_data.t()) / (num_samples - 1.0);
    covariance_matrix
}
fn compute_eigen(matrix: Array2<f64>) -> Result<(Array<f64, Axis>, Array2<f64>), LinalgError> {
    // Compute eigenvalues and eigenvectors
    let (eigenvalues, eigenvectors) = matrix.eigh(UPLO::Upper)?;

    Ok((eigenvalues, eigenvectors))
}
