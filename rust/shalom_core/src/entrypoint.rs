use std::{
    collections::HashMap,
    fs,
    path::{Path, PathBuf},
};

use crate::{
    context::{ShalomGlobalContext, SharedShalomGlobalContext, load_config_from_yaml}, // ✅ added config loader
    operation::context::SharedOpCtx,
    schema::{self, context::SharedSchemaContext},
};

pub struct FoundGqlFiles {
    pub schema: PathBuf,
    pub operations: Vec<PathBuf>,
}

//pub fn find_graphql_files(pwd: &Path) -> FoundGqlFiles {
//    let mut found_files = vec![];
//    found_files.extend(
//        glob::glob(pwd.join("**/*.graphql").to_str().unwrap())
//            .into_iter()
//            .flatten(),
//    );
//    found_files.extend(
//        glob::glob(pwd.join("**/*.gql").to_str().unwrap())
//            .into_iter()
//            .flatten(),
//    );
//    let mut schema = None;
//    let mut operations = vec![];
//    for file in found_files {
//        let file = file.unwrap();
//        let f_name = file.file_name().unwrap().to_str().unwrap();
//        if f_name.contains("schema.graphql") || f_name.contains("schema.gql") {
//            schema = Some(file);
//        } else {
//            operations.push(file);
//        }
//    }
//    FoundGqlFiles {
//        schema: schema.expect("No schema.graphql file found"),
//        operations,
//    }
//}
pub fn find_graphql_files(pwd: &Path) -> FoundGqlFiles {
    println!("[DEBUG] Searching for GraphQL files in: {}", pwd.display());

    let mut found_files = vec![];

    let graphql_glob = pwd.join("**/*.graphql").to_str().unwrap().to_string();
    let gql_glob = pwd.join("**/*.gql").to_str().unwrap().to_string();

    println!("[DEBUG] Glob patterns: {}, {}", graphql_glob, gql_glob);

    found_files.extend(
        glob::glob(&graphql_glob)
            .expect("Invalid glob pattern for .graphql")
            .filter_map(Result::ok),
    );

    found_files.extend(
        glob::glob(&gql_glob)
            .expect("Invalid glob pattern for .gql")
            .filter_map(Result::ok),
    );

    println!("[DEBUG] Total files found: {}", found_files.len());

    let mut schema = None;
    let mut operations = vec![];

    for file in &found_files {
        let f_name = file.file_name().unwrap().to_str().unwrap();
        println!("[DEBUG] Found file: {}", f_name);

        if f_name.contains("schema.graphql") || f_name.contains("schema.gql") {
            println!("[DEBUG] Schema file detected: {}", file.display());
            schema = Some(file.clone());
        } else {
            operations.push(file.clone());
        }
    }

    if schema.is_none() {
        println!("[ERROR] No schema.graphql or schema.gql file found in {:?}", pwd);
        for file in &found_files {
            println!(" -> Candidate file: {}", file.display());
        }
        panic!("No schema.graphql file found");
    }

    FoundGqlFiles {
        schema: schema.unwrap(),
        operations,
    }
}



pub fn parse_schema(schema: &str) -> anyhow::Result<SharedSchemaContext> {
    schema::resolver::resolve(schema)
}

pub fn parse_document(
    global_ctx: &SharedShalomGlobalContext,
    operation: &str,
    source_path: &PathBuf,
) -> anyhow::Result<HashMap<String, SharedOpCtx>> {
    crate::operation::parse::parse_document(global_ctx, operation, source_path)
}

pub fn parse_directory(pwd: &Path) -> anyhow::Result<SharedShalomGlobalContext> {
    let files = find_graphql_files(pwd);
    let schema_raw = fs::read_to_string(&files.schema)?;
    let schema_parsed = parse_schema(&schema_raw)?;

    // ✅ load shalom.yml
    let config = load_config_from_yaml("shalom.yml");

    // ✅ pass config into global context
    let global_ctx = ShalomGlobalContext::new(schema_parsed, config);

    for operation in files.operations {
        let content = fs::read_to_string(&operation).unwrap();
        let parsed = parse_document(&global_ctx, &content, &operation);
        global_ctx.register_operations(parsed.unwrap());
    }

    Ok(global_ctx)
}
