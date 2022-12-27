#[macro_use]


pub mod image;
pub mod masking;
pub mod statistics;
pub mod commands {
    pub mod base;
    pub mod parse;
    pub mod maskhemi;
    pub mod tsnr;
}


// rust or third party modules
use clap::Parser;
use simple_logger;
use log::info;


// own modules
use commands::{
    parse::{NirustArgs, ActionType},
    base::ExecutableCommand
};


fn main() {
    simple_logger::SimpleLogger::new().env().init().unwrap();
    info!("Starting nirust...");
    let args = NirustArgs::parse();
    
    match args.action_type {
        ActionType::MaskHemi(cmd) => cmd.execute(),
        ActionType::TemporalSNR(cmd) => cmd.execute()
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
