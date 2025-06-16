use serde::{Deserialize, Serialize};
use std::{collections::HashMap, path::PathBuf};

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct CustomScalarDefinition {
    pub graphql_name: String,
    pub scalar_dart_type: String,
    pub impl_symbol: RuntimeSymbolDefinition,
}

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct RuntimeSymbolDefinition {
    pub import_path: PathBuf,
    pub symbol_name: String,
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ShalomConfig {
    pub custom_scalars: HashMap<String, CustomScalarDefinition>,
}
