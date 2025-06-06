use serde::Deserialize;

#[derive(Debug, Deserialize)]
pub struct ScalarMapping {
    pub graphql_name: String,
    pub scalar_dart_type: String,
    pub impl_path: String, // lib/src/point.dart#pointScalarImpl
}

#[derive(Debug, Deserialize)]
pub struct ShalomConfig {
    pub scalars: Vec<ScalarMapping>,
}
