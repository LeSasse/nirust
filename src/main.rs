#[macro_use]

pub mod image;
pub mod commands;
pub mod masking;
pub mod statistics;

// rust or third party modules
use clap::Parser;
use commands::ExecutableCommand;
use log::info;

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
