//! All CLI commands are implemented in the `nirust::commands` module.
//! Specifically, each command is implemented as a struct with fields
//! corresponding to the arguments. Each command struct needs to implement the
//! `execute` method.

use clap::{Args, Parser, Subcommand};

use log::info;
use ndarray::prelude::*;
use std::path::Path;

use crate::{
    image::{get_affine, load_img, resample_3d_nifti, save_img},
    masking::{mask_hemi, parcellate},
    statistics::voxelwise_tsnr,
};

// For every command, the trait ExecutableCommand should be implemented by
// writing an associated method 'execute'
pub trait ExecutableCommand {
    fn execute(&self);
}

// Define the Argument Parser
#[derive(Debug, Parser)]
#[clap(
    author = "Leonard Sasse",
    version,
    about = "A collection of command line utilities for neuroimaging"
)]
pub struct NirustArgs {
    #[arg(short, long)]
    /// Whether to run verbose.
    pub verbose: bool,
    #[clap(subcommand)]
    pub action_type: ActionType,
}

// List and document each subcommand using clap
#[derive(Debug, Subcommand)]
pub enum ActionType {
    /// Mask the left or right hemisphere of a NIfTI image.
    MaskHemi(MaskHemiCommand),

    /// Compute the voxel-wise temporal SNR of a 4D NIfTI image.
    TemporalSNR(TemporalSNRCommand),

    /// Parcellate a 3D or 4D NIfTI image.
    Parcellate(ParcellateCommand),

    /// Resample a 3D NIfTI image to another 3D or 4D reference image
    /// using nearest neighbour interpolation.
    ResampleToImage(ResampleToImageCommand),
}

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
        info!("Running mask-hemi command...");
        match self.side.as_str() {
            "left" => info!("Masking left hemisphere..."),
            "right" => info!("Masking right hemisphere ..."),
            _ => panic!("Error: 'side' parameter can be 'left' or 'right'!"),
        }

        // function that loads image and returns header and ndarray
        let (header, mut image_data) = load_img(Path::new(&self.input_nifti));

        // function that does masking and returns header and ndarray
        mask_hemi(&header, &mut image_data, &self.side);

        // function that saves header and ndarray to a nifti
        save_img(Path::new(&self.output_nifti), &header, image_data);
    }
}

#[derive(Debug, Args)]
pub struct TemporalSNRCommand {
    /// 4D NIfTI for which to compute voxel-wise tSNR.
    pub input_nifti: String,
    /// Path to store the voxel-wise tSNR as a NIfTI image.
    pub output_nifti: String,
}

impl ExecutableCommand for TemporalSNRCommand {
    fn execute(&self) {
        let (header, image_data) = load_img(Path::new(&self.input_nifti));
        let image_data = voxelwise_tsnr(image_data);
        info!("Saving tSNR NIfTI image at {}", self.output_nifti);
        save_img(Path::new(&self.output_nifti), &header, image_data);
    }
}

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

        let (header_parc, parc_data) =
            load_img(Path::new(&self.parcellation_nifti));

        let x = parc_data.shape()[0];
        let y = parc_data.shape()[1];
        let z = parc_data.shape()[2];
        let shape = (x, y, z);
        let parc_data_shaped: Array<f32, Ix3> =
            parc_data.into_shape(shape).unwrap();

        let parcellated = parcellate(
            &image_data,
            &header_img,
            &parc_data_shaped,
            &header_parc,
        );
        println!("{:?}", parcellated);
    }
}

#[derive(Debug, Args)]
pub struct ResampleToImageCommand {
    /// 3D NIfTI to resample.
    pub input_nifti: String,
    /// Reference NIfTI image as resampling target.
    pub reference_nifti: String,
    /// Output path.    
    pub output_nifti: String,
}

impl ExecutableCommand for ResampleToImageCommand {
    fn execute(&self) {
        let input_nifti = Path::new(&self.input_nifti);
        let ref_nifti = Path::new(&self.reference_nifti);
        let (header, image_data) = load_img(input_nifti);
        let (ref_header, ref_image) = load_img(ref_nifti);

        let i = image_data.shape()[0];
        let j = image_data.shape()[1];
        let k = image_data.shape()[2];

        let image_data_shaped: Array<f32, Ix3> =
            image_data.into_shape((i, j, k)).unwrap();

        let target_shape = ref_image.shape();
        let x = target_shape[0];
        let y = target_shape[1];
        let z = target_shape[2];

        let image_affine = get_affine(&header);
        let ref_affine = get_affine(&ref_header);
        let data_resampled = resample_3d_nifti(
            &image_data_shaped,
            &image_affine,
            &ref_affine,
            (x, y, z),
        );
        save_img(
            Path::new(&self.output_nifti),
            &ref_header,
            data_resampled.into_dyn(),
        );
    }
}
