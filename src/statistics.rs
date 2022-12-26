
use ndarray::prelude::*;
use std::f32::NAN;
use nifti::{
    writer::WriterOptions,
    error::NiftiError,
    NiftiObject,
    ReaderOptions,
    NiftiVolume,
    IntoNdArray
};
use log::{info, warn};

use crate::image::squeeze_header;

pub fn voxelwise_tsnr(
    input_nifti: String,
    output_nifti: String
) -> Result<(), NiftiError> {
    
    info!("Reading NIfTI at {}", input_nifti);
    let img = ReaderOptions::new().read_file(input_nifti)?;
    
    let header = img.header().clone();
    let volume = img.volume();
    let dims = volume.dim();
    let n_dims = dims.len();
    
    // Validate input
    if n_dims != 4 {
        panic!(
            r"Temporal SNR can only be calculated 
            for images with a temporal dimension."
        );
    }
    
    let image_data = img.into_volume().into_ndarray::<f32>().unwrap();
    
    info!("Calculating voxel-wise mean along time axis...");
    let mean_img = image_data.mean_axis(Axis(3)).unwrap();
    info!("{:?}", mean_img.shape());
    
    info!("Calculating voxel-wise standard deviation along time axis...");
    let std_img = image_data.std_axis(Axis(3), 1.);
    info!("{:?}", std_img.shape());
    
    info!("Calculating tSNR...");
    let tsnr_img = mean_img / std_img;
    
    let updated_header = squeeze_header(&header);
    
    info!("Saving tSNR NIfTI image at {}", output_nifti);
    match WriterOptions::new(output_nifti).reference_header(
        &updated_header
    ).write_nifti(&tsnr_img) {
        Ok(()) => {}
        Err(e) => {
            panic!("Error: {}", e);
        }
    }
        
    Ok(())
}
