
use clap::Args;
use std::path::Path;
use log::info;

use crate::{
    commands::base::ExecutableCommand,
    image::{load_img, save_img},
    masking::mask_hemi,
};

#[derive(Debug, Args)]
pub struct MaskHemiCommand {
    /// NIfTI file to mask
    pub input_nifti: String,
    /// Path to store masked NIfTI
    pub output_nifti: String,
    /// Mask 'left' or 'right' hemisphere.
    pub side: String,
}


impl ExecutableCommand for MaskHemiCommand { 
    fn execute(&self) {
        
        match self.side.as_str() {
            "left" => info!("Masking left hemisphere..."),
            "right" => info!("Masking right hemisphere ..."),
            _ => panic!("Error: 'side' parameter can be 'left' or 'right'!")
        }

        info!("Running mask-hemi command...");        
        // function that loads image and returns header and ndarray
        let (header, image_data) = load_img(&Path::new(&self.input_nifti));
                
        // function that does masking and returns header and ndarray
        let image_data = mask_hemi(&header, image_data, &self.side);
   
        // function that saves header and ndarray to a nifti
        save_img(&Path::new(&self.output_nifti), &header, image_data);
        
    }
}

