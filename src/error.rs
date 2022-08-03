use thiserror::Error;

use crate::utils::PackageManager;

pub type Result = std::result::Result<(), Error>;

#[derive(Error, Debug)]
pub enum Error {
    #[error("rustfmt failed: {0}")]
    Rustfmt(String),
    #[error("dart fmt failed: {0}")]
    Dartfmt(String),
    #[error(
        "ffigen could not find LLVM.
    Please supply --llvm-path to flutter_rust_bridge_codegen, e.g.:
    
        flutter_rust_bridge_codegen .. --llvm-path <path_to_llvm>"
    )]
    FfigenLlvm,
    #[error("{0} is not a command, or not executable.")]
    MissingExe(String),
    #[error("{0}")]
    StringError(String),
    #[error("please add {name} to your {manager}. (version {requirement})")]
    MissingDep {
        name: String,
        manager: PackageManager,
        requirement: String,
    },
    #[error("please update version of {name} in your {manager}. (version {requirement})")]
    InvalidDep {
        name: String,
        manager: PackageManager,
        requirement: String,
    },
}

impl Error {
    pub fn str(msg: &str) -> Self {
        Self::StringError(msg.to_owned())
    }

    pub fn string(msg: String) -> Self {
        Self::StringError(msg)
    }
}
