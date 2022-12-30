
use clap::{
    Parser,
    Subcommand
};

use crate::commands::{
    maskhemi::MaskHemiCommand,
    tsnr::TemporalSNRCommand,
    parcellate::ParcellateCommand,
};


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
}
