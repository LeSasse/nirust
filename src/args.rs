
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
    
    /// Mask the left or right hemisphere of the nifti image.
    MaskHemi(MaskHemiCommand),
    
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