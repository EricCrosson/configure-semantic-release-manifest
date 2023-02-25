use std::iter::FromIterator;
use std::path::PathBuf;
use std::{collections::HashSet, path::Path};

use clap::Parser;

use configure_semantic_release_manifest::{SemanticReleaseConfiguration, WriteTo};
use find_semantic_release_config::find_semantic_release_configuration;

type Error = Box<dyn std::error::Error>;
type Result<T> = core::result::Result<T, Error>;

const SUPPORTED_FILE_TYPES: &[&str] = &["json"];

#[derive(Parser, Debug)]
#[command(author, version, about)]
struct Cli {
    /// Directory in which to search for semantic-release manifest
    #[arg(long, default_value = ".")]
    directory: PathBuf,

    /// Remove a plugin's configuration
    #[arg(long, action=clap::ArgAction::Append)]
    remove: Vec<String>,

    /// Edit file in-place
    #[arg(long, action)]
    in_place: bool,
}

fn find_semantic_release_config(directory: &Path) -> Result<PathBuf> {
    Ok(find_semantic_release_configuration(&directory)?.ok_or_else(
        || -> Box<dyn std::error::Error> {
            format!(
                "unable to find semantic-release configuration in {:?}",
                &directory,
            )
            .into()
        },
    )?)
}

fn is_unsupported_file_extension(config: &Path) -> bool {
    match config.extension() {
        Some(extension) => {
            let extension = extension.to_string_lossy();
            !SUPPORTED_FILE_TYPES
                .iter()
                .any(|supported_extension| &extension.as_ref() == supported_extension)
        }
        None => false,
    }
}

fn main() -> Result<()> {
    env_logger::Builder::from_env(env_logger::Env::default().default_filter_or("warn")).init();

    let cli = Cli::parse();
    let config = find_semantic_release_config(&cli.directory)?;

    if is_unsupported_file_extension(&config) {
        eprintln!(
            "Error: unsupported file extension {:?}",
            config.extension().unwrap_or_default()
        );
        eprintln!("Currently configure-semantic-release-manifest only supports the following extensions: {:?}", SUPPORTED_FILE_TYPES);
        return Err("unsupported file extension".into());
    }

    let mut configuration = SemanticReleaseConfiguration::read_from_file(&config)?;

    configuration.remove_plugin_configuration(HashSet::from_iter(cli.remove))?;
    match cli.in_place {
        true => configuration.write_if_modified(WriteTo::InPlace)?,
        false => configuration.write_if_modified(WriteTo::Stdout)?,
    };

    Ok(())
}
