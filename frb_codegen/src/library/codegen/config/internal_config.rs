use crate::codegen::generator::api_dart::internal_config::GeneratorApiDartInternalConfig;
use crate::codegen::generator::wire::c::internal_config::GeneratorWireCInternalConfig;
use crate::codegen::generator::wire::dart::internal_config::GeneratorWireDartInternalConfig;
use crate::codegen::generator::wire::rust::internal_config::GeneratorWireRustInternalConfig;
use crate::codegen::parser::internal_config::ParserInternalConfig;
use crate::codegen::polisher::internal_config::PolisherInternalConfig;
use crate::codegen::preparer::internal_config::PreparerInternalConfig;
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct InternalConfig {
    pub preparer: PreparerInternalConfig,
    pub parser: ParserInternalConfig,
    pub generator: GeneratorInternalConfig,
    pub polisher: PolisherInternalConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorInternalConfig {
    pub api_dart: GeneratorApiDartInternalConfig,
    pub wire: GeneratorWireInternalConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct GeneratorWireInternalConfig {
    pub dart: GeneratorWireDartInternalConfig,
    pub rust: GeneratorWireRustInternalConfig,
    pub c: GeneratorWireCInternalConfig,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq)]
pub(crate) struct RustInputPathPack {
    pub rust_input_path: HashMap<Namespace, PathBuf>,
}

// TODO move?
#[derive(Debug, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
#[serde(transparent)]
pub struct Namespace {
    pub name: String,
}

impl From<String> for Namespace {
    fn from(name: String) -> Self {
        Self { name }
    }
}
