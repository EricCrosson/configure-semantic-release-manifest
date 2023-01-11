use std::collections::HashSet;
use std::iter::FromIterator;
use std::path::PathBuf;

use clap::Parser;

use configure_semantic_release_manifest::{SemanticReleaseConfiguration, WriteTo};

const SEMANTIC_RELEASE_MANIFEST_PATH: &'static str = ".releaserc.json";

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    #[arg(long, default_value = SEMANTIC_RELEASE_MANIFEST_PATH)]
    config: PathBuf,

    /// Remove
    #[arg(long, action=clap::ArgAction::Append)]
    remove: Vec<String>,

    /// Edit file in-place
    #[arg(long, action)]
    in_place: bool,
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let cli = Cli::parse();

    let mut configuration = SemanticReleaseConfiguration::read_from_file(&cli.config)?;
    configuration.remove_plugin_configuration(HashSet::from_iter(cli.remove));
    match cli.in_place {
        true => configuration.write_if_modified(WriteTo::InPlace)?,
        false => configuration.write_if_modified(WriteTo::Stdout)?,
    };

    Ok(())
}
