use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct CostumeScalarDefinition {
    pub graphql_name: String,
    pub scalar_dart_type: String,
    pub impl_symbol: (PathBuf, String),
}

#[derive(Debug, Deserialize)]
pub struct ShalomConfig {
    pub scalars: HashMap<String, ScalarMapping>,
}
