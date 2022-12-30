
use clap::Args;
use std::path::Path;
use log::info;
use ndarray::prelude::*;

use crate::{
    commands::base::ExecutableCommand,
    image::load_img,
    masking::parcellate,
};


#[derive(Debug, Args)]
pub struct ParcellateCommand {
    /// NIfTI file to parcellate.
    pub input_nifti: String,
    /// NIfTI file with parcellation scheme.
    pub parcellation_nifti: String,
    /// Path to output .tsv file.
    pub output_tsv: String,
}


impl ExecutableCommand for ParcellateCommand {
    fn execute(&self) {
        info!("Running parcellate command...");
    
        let (header_img, image_data) = load_img(Path::new(&self.input_nifti));
        
        let (header_parc, parc_data) = load_img(
            Path::new(&self.parcellation_nifti)
        );
          
        let x = parc_data.shape()[0];
        let y = parc_data.shape()[1];
        let z = parc_data.shape()[2];
        let shape = (x, y, z);
        let parc_data_shaped: Array<f32, Ix3> = parc_data
            .into_shape(shape)
            .unwrap();

        parcellate(
            &image_data,
            &header_img,
            &parc_data_shaped,
            &header_parc,
        );
        //println!("{:?}", parcellated.shape());
    }
}