
use clap::{
    Parser,
    Subcommand
};

use crate::commands::{
    maskhemi::MaskHemiCommand,
    tsnr::TemporalSNRCommand,
};

// Define the Argument Parser
#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct NirustArgs {
    #[clap(subcommand)]
    pub action_type: ActionType,
}


// List and document each subcommand using clap
#[derive(Debug, Subcommand)]
pub enum ActionType {
    
    /// Mask the left or right hemisphere of a NIfTI image.
    MaskHemi(MaskHemiCommand),
    
    // Compute the voxel-wise temporal SNR of a 4D NIfTI image.
    TemporalSNR(TemporalSNRCommand),   
}
