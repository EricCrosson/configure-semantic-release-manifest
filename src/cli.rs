use std::path::PathBuf;

use clap::Parser;

#[derive(Parser, Debug)]
#[command(author, version, about)]
pub(crate) struct Cli {
    /// Directory in which to search for semantic-release manifest
    #[arg(long, default_value = ".")]
    pub directory: PathBuf,

    /// Remove a plugin's configuration
    #[arg(long, action=clap::ArgAction::Append)]
    pub remove: Vec<String>,

    /// Edit file in-place
    #[arg(long, action)]
    pub in_place: bool,
}
