

use clap::Args;
use std::path::Path;
use log::info;

use crate::{
    commands::base::ExecutableCommand,
    image::{load_img, save_img},
    statistics::voxelwise_tsnr,
};


#[derive(Debug, Args)]
pub struct TemporalSNRCommand {
    /// 4D NIfTI for which to compute voxel-wise tSNR.
    pub input_nifti: String,
    /// Path to store the voxel-wise tSNR as a NIfTI image.
    pub output_nifti: String
}

impl ExecutableCommand for TemporalSNRCommand {
    fn execute(&self) {
        let (header, image_data) = load_img(&Path::new(&self.input_nifti));
        let image_data = voxelwise_tsnr(image_data);
        info!("Saving tSNR NIfTI image at {}", self.output_nifti);
        save_img(&Path::new(&self.output_nifti), &header, image_data);
    }
}