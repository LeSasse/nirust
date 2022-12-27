
use ndarray::prelude::*;
use log::info;


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
    let tsnr_img = mean_img / std_img;
        
    tsnr_img
}
