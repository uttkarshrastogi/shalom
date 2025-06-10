use serde::{Deserialize, Deserializer};
use std::{collections::HashMap, path::PathBuf};

#[derive(Debug, Deserialize)]
pub struct CostumeScalarDefinition {
    pub graphql_name: String,
    pub scalar_dart_type: String,

    #[serde(deserialize_with = "tuple_from_seq")]
    pub impl_symbol: (PathBuf, String),
}

#[derive(Debug, Deserialize)]
pub struct ShalomConfig {
    pub scalars: HashMap<String, CostumeScalarDefinition>,
}

/// Custom deserializer to treat YAML list as tuple
fn tuple_from_seq<'de, D>(deserializer: D) -> Result<(PathBuf, String), D::Error>
where
    D: Deserializer<'de>,
{
    let v: Vec<String> = Deserialize::deserialize(deserializer)?;
    if v.len() != 2 {
        return Err(serde::de::Error::custom(
            "Expected array of 2 elements for impl_symbol",
        ));
    }
    Ok((PathBuf::from(&v[0]), v[1].clone()))
}
