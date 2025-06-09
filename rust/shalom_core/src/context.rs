use std::{
    collections::HashMap,
    fs,
    sync::{Arc, Mutex},
};
use crate::shalom_config::{ShalomConfig, ScalarMapping};

use serde::Deserialize;

use crate::{operation::context::SharedOpCtx, schema::context::SharedSchemaContext};


#[derive(Debug)]
pub struct ShalomGlobalContext {
    operations: Mutex<HashMap<String, SharedOpCtx>>,
    pub schema_ctx: SharedSchemaContext,
    pub config: ShalomConfig,

}

unsafe impl Send for ShalomGlobalContext {}
unsafe impl Sync for ShalomGlobalContext {}

impl ShalomGlobalContext {

    pub fn new(schema_ctx: SharedSchemaContext, config: ShalomConfig) -> Arc<Self> {
        Arc::new(Self {
            operations: Mutex::new(HashMap::new()),
            schema_ctx,
            config,
        })
    }

    pub fn register_operations(&self, operations_update: HashMap<String, SharedOpCtx>) {
        let mut operations = self.operations.lock().unwrap();
        for (name, _) in operations_update.iter() {
            if operations.contains_key(name) {
                panic!("Operation with name {} already exists", name);
            }
        }
        operations.extend(operations_update);
    }

    pub fn operations(&self) -> Vec<(String, SharedOpCtx)> {
        let operations = self.operations.lock().unwrap();
        operations
            .iter()
            .map(|(name, op)| (name.clone(), op.clone()))
            .collect()
    }


    pub fn find_scalar(&self, graphql_name: &str) -> Option<&ScalarMapping> {
        self.config
            .scalars
            .iter()
            .find(|s| s.graphql_name == graphql_name)
    }

    pub fn operation_exists(&self, name: &str) -> bool {
        let operations = self.operations.lock().unwrap();
        operations.contains_key(name)
    }
}

pub type SharedShalomGlobalContext = Arc<ShalomGlobalContext>;

pub fn load_config_from_yaml(path: &str) -> ShalomConfig {
    let content = fs::read_to_string(path).expect("Failed to read shalom.yml");
    serde_yaml::from_str(&content).expect("Failed to parse shalom.yml")
}
