
use clap:: {
    Args,
    Parser,
    Subcommand
};

#[derive(Debug, Parser)]
#[clap(author, version, about)]
pub struct NirustArgs {
    #[clap(subcommand)]
    pub action_type: ActionType,
}

#[derive(Debug, Subcommand)]
pub enum ActionType {
    
    /// Mask the left or right hemisphere of a NIfTI image.
    MaskHemi(MaskHemiCommand),
    
    /// Compute the voxel-wise temporal SNR of a 4D NIfTI image.
    TemporalSNR(TemporalSNRCommand),   
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

#[derive(Debug, Args)]
pub struct TemporalSNRCommand {
    /// 4D NIfTI for which to compute voxel-wise tSNR.
    pub input_nifti: String,
    /// Path to store the voxel-wise tSNR as a NIfTI image.
    pub output_nifti: String, 
}