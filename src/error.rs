use std::{
    io,
    path::{Path, PathBuf},
};

#[derive(Debug)]
pub enum Error {
    /// Could not find semantic-release configuration
    ConfigurationFileNotFound { path: PathBuf },

    /// Unable to open file
    FileOpenError { source: io::Error, path: PathBuf },

    /// Unable to read file
    FileReadError { source: io::Error, path: PathBuf },

    /// Unable to parse semantic-release configuration file
    FileParseError { source: serde_json::Error },

    /// Unexpected contents in semantic-release configuration file
    UnexpectedContentsError,

    /// Unable to serialize file
    FileSerializeError { source: serde_json::Error },

    /// Unable to write changes to file
    FileWriteError { source: io::Error, path: PathBuf },
}

impl std::error::Error for Error {
    fn source(&self) -> Option<&(dyn std::error::Error + 'static)> {
        match self {
            Error::ConfigurationFileNotFound { path: _ } => None,
            Error::FileOpenError { source, path: _ } => Some(source),
            Error::FileReadError { source, path: _ } => Some(source),
            Error::FileParseError { source } => Some(source),
            Error::UnexpectedContentsError => None,
            Error::FileSerializeError { source } => Some(source),
            Error::FileWriteError { source, path: _ } => Some(source),
        }
    }
}

impl std::fmt::Display for Error {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            Error::ConfigurationFileNotFound { path } => {
                write!(
                    f,
                    "Could not find semantic-release configuration in {:?}",
                    path
                )
            }
            Error::FileOpenError { source: _, path } => {
                write!(f, "Unable to open file {:?}", path)
            }
            Error::FileReadError { source: _, path } => {
                write!(f, "Unable to read file {:?}", path)
            }
            Error::FileParseError { source: _ } => {
                write!(f, "Unable to parse semantic-release configuration file")
            }
            Error::UnexpectedContentsError => {
                write!(
                    f,
                    "Unexpected contents in semantic-release configuration file"
                )
            }
            Error::FileSerializeError { source: _ } => {
                write!(f, "Unable to serialize file")
            }
            Error::FileWriteError { source: _, path } => {
                write!(f, "Unable to write changes to file {:?}", path)
            }
        }
    }
}

impl Error {
    pub(crate) fn configuration_file_not_found_error(path: &Path) -> Error {
        Error::ConfigurationFileNotFound {
            path: path.to_owned(),
        }
    }

    pub(crate) fn file_open_error(source: io::Error, path: &Path) -> Error {
        Error::FileOpenError {
            source,
            path: path.to_owned(),
        }
    }

    pub(crate) fn file_read_error(source: io::Error, path: &Path) -> Error {
        Error::FileReadError {
            source,
            path: path.to_owned(),
        }
    }

    pub(crate) fn file_parse_error(source: serde_json::Error) -> Error {
        Error::FileParseError { source }
    }

    pub(crate) fn file_serialize_error(source: serde_json::Error) -> Error {
        Error::FileSerializeError { source }
    }

    pub(crate) fn file_write_error(source: io::Error, path: &Path) -> Error {
        Error::FileWriteError {
            source,
            path: path.to_owned(),
        }
    }
}
