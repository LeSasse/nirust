#[macro_use]

pub mod image;
pub mod masking;
pub mod statistics;
pub mod commands {
    pub mod base;
    pub mod parse;
    pub mod maskhemi;
    pub mod tsnr;
    pub mod parcellate;
}


// rust or third party modules
use clap::Parser;

use log::info;


// own modules
use commands::{
    parse::{NirustArgs, ActionType},
    base::ExecutableCommand
};


fn main() {
    let args = NirustArgs::parse();

    if args.verbose {
        simple_logger::SimpleLogger::new()
            .env()
            .init()
            .unwrap();
        info!("Starting nirust...");
    }
        
    match args.action_type {
        ActionType::MaskHemi(cmd) => cmd.execute(),
        ActionType::TemporalSNR(cmd) => cmd.execute(),
        ActionType::Parcellate(cmd) => cmd.execute(),
    }
    
            
    // match args.action_type {
    //     ActionType::MaskHemi(
    //         MaskHemiCommand { input_nifti, output_nifti, side }
    //     ) => {
    //         match masking::mask_hemi(input_nifti, output_nifti, &side) {
    //             Ok(()) => {},
    //             Err(e) => {
    //                 panic!("Error: {}", e);
    //             }
    //         }
    //     },
    //     ActionType::TemporalSNR(
    //         TemporalSNRCommand { input_nifti, output_nifti }
    //     ) => {
    //         match statistics::voxelwise_tsnr(input_nifti, output_nifti) {
    //             Ok(()) => {},
    //             Err(e) => {
    //                 panic!("{}", e)
    //             }
    //         }
    //     }
    // }
}
