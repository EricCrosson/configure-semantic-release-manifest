use std::{
    collections::HashSet,
    io::{BufWriter, Write},
    path::PathBuf,
};
#[forbid(unsafe_code)]
use std::{fs::File, io::Read, path::Path};

use log::debug;
use serde::{Deserialize, Serialize};

mod error;

use crate::error::Error;

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ModifiedFlag {
    Unmodified,
    Modified,
}

#[derive(Clone, Debug, Deserialize, Serialize)]
#[serde(untagged)]
pub enum SemanticReleasePluginConfiguration {
    WithoutConfiguration(String),
    WithConfiguration(Vec<serde_json::Value>),
}

#[derive(Clone, Debug, Deserialize, Serialize)]
pub struct SemanticReleaseManifest {
    pub plugins: Option<Vec<SemanticReleasePluginConfiguration>>,
}

pub struct SemanticReleaseConfiguration {
    manifest: SemanticReleaseManifest,
    manifest_path: PathBuf,
    dirty: ModifiedFlag,
}

impl SemanticReleasePluginConfiguration {
    pub fn plugin_name(&self) -> Option<&str> {
        match self {
            SemanticReleasePluginConfiguration::WithoutConfiguration(value) => Some(value),
            SemanticReleasePluginConfiguration::WithConfiguration(array) => {
                array.get(0).and_then(|value| value.as_str())
            }
        }
    }
}

impl SemanticReleaseManifest {
    pub fn parse_from_string(string: &str) -> Result<Self, Error> {
        serde_json::from_str(&string).map_err(|err| Error::file_parse_error(err))
    }

    pub fn remove_plugin_configuration(&mut self, to_remove: HashSet<String>) -> ModifiedFlag {
        let mut dirty = ModifiedFlag::Unmodified;
        // let plugins_to_remove: HashSet<String> = cli.remove.into_iter().collect();

        if let Some(plugins) = self.plugins.clone() {
            let filtered_plugins: Vec<_> = plugins
                .into_iter()
                .filter(|configuration| {
                    match configuration.plugin_name() {
                        Some(plugin_name) => {
                            if to_remove.contains(plugin_name) {
                                debug!("Removing configuration for plugin {}", plugin_name);
                                dirty = ModifiedFlag::Modified;
                                return false;
                            } else {
                                return true;
                            }
                        }
                        // Not a valid plugin, so leave it unchanged
                        None => true,
                    }
                })
                .collect();

            self.plugins = Some(filtered_plugins);
        }

        dirty
    }
}

impl std::fmt::Display for SemanticReleaseManifest {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", serde_json::to_string_pretty(self).unwrap())
    }
}

impl SemanticReleaseConfiguration {
    // DISCUSS: could pull out a library for finding the semantic-release manifest.
    // This sounds a lot like is-semantic-release-configured
    // For now, assume it's .releaserc.json and document the limitation
    pub fn read_from_file(semantic_release_manifest_path: &Path) -> Result<Self, Error> {
        debug!(
            "Reading semantic-release configuration from file {:?}",
            semantic_release_manifest_path
        );

        if !semantic_release_manifest_path.exists() {
            return Err(Error::configuration_file_not_found_error(
                semantic_release_manifest_path,
            ));
        }

        // Reading a file into a string before invoking Serde is faster than
        // invoking Serde from a BufReader, see
        // https://github.com/serde-rs/json/issues/160
        let mut string = String::new();
        File::open(semantic_release_manifest_path)
            .map_err(|err| Error::file_open_error(err, semantic_release_manifest_path))?
            .read_to_string(&mut string)
            .map_err(|err| Error::file_read_error(err, semantic_release_manifest_path))?;

        Ok(Self {
            manifest: SemanticReleaseManifest::parse_from_string(&string)?,
            manifest_path: semantic_release_manifest_path.to_owned(),
            dirty: ModifiedFlag::Unmodified,
        })
    }

    fn write(&self) -> Result<(), Error> {
        debug!(
            "Writing semantic-release configuration to file {:?}",
            self.manifest_path
        );
        let file = File::create(&self.manifest_path)
            .map_err(|err| Error::file_open_error(err, &self.manifest_path))?;
        let mut writer = BufWriter::new(file);
        serde_json::to_writer_pretty(&mut writer, &self.manifest)
            .map_err(|err| Error::file_serialize_error(err))?;
        writer
            .write_all(b"\n")
            .map_err(|err| Error::file_write_error(err, &self.manifest_path))?;
        writer
            .flush()
            .map_err(|err| Error::file_write_error(err, &self.manifest_path))?;

        Ok(())
    }

    pub fn write_if_modified(&self) -> Result<(), Error> {
        match self.dirty {
            ModifiedFlag::Unmodified => Ok(()),
            ModifiedFlag::Modified => self.write(),
        }
    }

    pub fn remove_plugin_configuration(&mut self, to_remove: HashSet<String>) {
        let modified = self.manifest.remove_plugin_configuration(to_remove);
        if modified == ModifiedFlag::Modified {
            self.dirty = ModifiedFlag::Modified;
        }
    }
}
