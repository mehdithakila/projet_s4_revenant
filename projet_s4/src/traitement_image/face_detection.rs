use image::*;
use super::convert_to_grey::*;
use ndarray::*;
use ndarray_linalg::*;
use ndarray_linalg::error::LinalgError;
use ndarray::ArrayBase;
use ndarray::OwnedRepr;
use ndarray::{Array3};
use super::redim::*;
pub fn conv_mat(images:Vec<DynamicImage>)->Array2<f64>
{
    /*let images=Vec::new();
    for path in path_images
    {
        let img=redim(&path);
        images.push(img);
    }*/
    let mut data:Vec<Array2<f64>> = Vec::new();
    let (width,height)=images[0].dimensions();
    let l=images.len();
    for image in images
    {
        let grey_image=convert_to_grey(&image);
        let mut flattened_vec: Vec<f64> = Vec::with_capacity((width * height) as usize);

         // Iterate over each pixel in the grayscale image
        for y in 0..height 
        {
            for x in 0..width 
            {   
                // Get the pixel value (u8) at the current position
                let pixel_value = grey_image.get_pixel(x, y)[0] as f64;

                // Push the pixel value to the flattened vector
                flattened_vec.push(pixel_value);
            }
        }
        let img_matrix = Array2::from_shape_vec((1, flattened_vec.len()), flattened_vec).unwrap();

        data.push(img_matrix);     
    }
    let mut stacked_array: Array3<f64> = Array3::zeros((data.len(), data[0].raw_dim()[0], data[0].raw_dim()[0]));
    for (index, array2) in data.iter().enumerate() {
        stacked_array.slice_mut(s![index, .., ..]).assign(array2);
    }
    // Compute the mean vector
    let mean = stacked_array.mean_axis(Axis(0)).unwrap();

    // Center the data by subtracting the mean vector
    let centered_data = stacked_array - &mean;
    let n_samples = centered_data.dim().0;
    let n_features = centered_data.dim().1;

    // Reshape centered_data into a 2D array
    let centered_data: Array2<f64> = centered_data.reversed_axes().into_shape((n_samples, n_features)).unwrap();
    let mut covariance_matrix = Array2::<f64>::zeros((n_samples, n_samples));
    // Compute the covariance matrix
    for i in 0..n_samples {
        for j in 0..=i {
            let cov = (centered_data.index_axis( Axis(0), i) )
                .dot(&(centered_data.index_axis( Axis(0), j)).t());
            covariance_matrix[(i, j)] = cov;
            covariance_matrix[(j, i)] = cov;
        }
    }
    // Compute the covariance matrix
    //
    covariance_matrix
}
fn compute_eigen(matrix: Array2<f64>) -> Result<(Array1<f64>, Array2<f64>), LinalgError> {
    // Compute eigenvalues and eigenvectors
    let (eigenvalues, eigenvectors) = matrix.eigh(UPLO::Upper)?;

    Ok((eigenvalues, eigenvectors))
}





