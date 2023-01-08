//! The `nirust::statistics` module provides functions that compute common
//! statistics describing an image (for example computing the temporal
//! signal-to-noise ratio using `voxelwise_tsnr`).

use log::info;
use ndarray::prelude::*;

/// Compute the temporal signal-to-noise ratio for every voxel
///
/// The temporal signal-to-noise ratio is defined as the mean signal divided
/// by the standard deviation of the signal.
///
/// Parameters
/// ----------
/// image_data : 4D ndarray containing the voxelwise image data, with the last
/// dimension corresponding to the time dimension. 
pub fn voxelwise_tsnr(image_data: Array<f32, IxDyn>) -> Array<f32, IxDyn> {
    let dims = image_data.shape();
    let n_dims = dims.len();

    // Validate input
    if n_dims != 4 {
        panic!(
            r"Temporal SNR can only be calculated 
            for images with a temporal dimension."
        );
    }

    info!("Calculating voxel-wise mean along time axis...");
    let mean_img = image_data.mean_axis(Axis(3)).unwrap();

    info!("Calculating voxel-wise standard deviation along time axis...");
    let std_img = image_data.std_axis(Axis(3), 1.);

    info!("Calculating tSNR...");

    mean_img / std_img
}
