use ndarray::{Array2, Axis,stack};
use ndarray_stats::DeviationExt;
use ndarray::s;
pub fn pca(data: &Array2<f64>, num_components: usize) -> Array2<f64> {
    // Step 1: Calculate the covariance matrix
    let centered_data = data - data.mean_axis(Axis(0)).unwrap().mean().unwrap(); // Center the data
    let covariance_matrix = centered_data.t().dot(&centered_data) / (data.rows() as f64);

    // Step 2: Compute Eigenvalues and Eigenvectors (Example: Using SVD)
    let (_, _, v) = covariance_matrix.svd(true, true).unwrap();
    let principal_components = v.slice(s![.., 0..num_components]);

    // Step 3: Transform Data
    centered_data.dot(&principal_components.t())
}

