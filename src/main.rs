//! # Overview
//! The `nirust` crate is a collection of commonly-used high level functions
//! for processing NIfTI images, performing statistical analyses, or extracting
//! features of interest from them. As such, it provides a
//! command-line-interface that can be used to quickly perform transformations
//! or analyes, which can come in handy during data discovery. You can look up
//! the CLI by running `nirust --help`. To further look up specific commands,
//! you can run `--help` for each command, for example: 
//! `nirust mask-hemi --help`.
//!
//! # Affine transformations
//! Since NIfTI images give information for three different potential affines 
//! (sform, qform, and fall-back affine), whenever a function (for example
//! resampling) requires affine transformations, the program will follow 
//! [the conventions outlined by the Python-based nibabel library](https://nipy.org/nibabel/nifti_images.html#choosing-the-image-affine):
//!
//! > 1. If sform_code != 0 (‘unknown’) use the sform affine; else
//! > 2. If qform_code != 0 (‘unknown’) use the qform affine; else
//! > 3. Use the fall-back affine.
//!
//! In general, nibabel has great documentation and some excellent tutorials
//! [on the NIfTI file format and affine transformations](https://nipy.org/nibabel/tutorials.html).
//! Therefore, if you are not familiar with these things, nibabel is a great
//! place to start learning about data processing in the field of neuroimaging.
//! You can also refer to the [official NIfTI file specifications for more information](https://nifti.nimh.nih.gov/pub/dist/src/niftilib/nifti1.h).


#[macro_use]

pub mod image;
pub mod commands;
pub mod masking;
pub mod statistics;

// rust or third party modules
use clap::Parser;
use commands::ExecutableCommand;
use log::info;


/// This is to document the main function.
fn main() {
    let args = commands::NirustArgs::parse();
    
    if args.verbose {
        simple_logger::SimpleLogger::new().env().init().unwrap();
        info!("Starting nirust...");
    }

    match args.action_type {
        commands::ActionType::MaskHemi(cmd) => cmd.execute(),
        commands::ActionType::TemporalSNR(cmd) => cmd.execute(),
        commands::ActionType::Parcellate(cmd) => cmd.execute(),
        commands::ActionType::ResampleToImage(cmd) => cmd.execute(),
    }
}
