use std::{cell::RefCell, rc::Rc};

use apollo_compiler::Node;
use serde::{Deserialize, Serialize};

use crate::schema::types::{EnumType, ScalarType};

/// the name of i.e object in a graphql query based on the parent fields.
pub type FullPathName = String;

#[derive(Debug, Serialize, Deserialize)]
#[serde(tag = "name")]
pub enum OperationType {
    Query,
    Mutation,
    Subscription,
}

/// common fields for selections
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct SelectionCommon {
    pub selection_name: String,
    pub is_optional: bool,
    pub full_name: FullPathName,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "kind")]
pub enum Selection {
    Scalar(Rc<ScalarSelection>),
    Object(Rc<ObjectSelection>),
    Enum(Rc<EnumSelection>),
}

impl Selection {
    pub fn self_selection_name(&self) -> String {
        match self {
            Selection::Scalar(node) => node.common.selection_name.clone(),
            Selection::Object(obj) => obj.common.selection_name.clone(),
            Selection::Enum(enum_) => enum_.common.selection_name.clone(),
        }
    }
    pub fn self_full_path_name(&self) -> &FullPathName {
        match self {
            Selection::Scalar(node) => &node.common.full_name,
            Selection::Object(obj) => &obj.common.full_name,
            Selection::Enum(_) => {
                panic!("enums dont have a full name as they are global per schema")
            }
        }
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ScalarSelection {
    #[serde(flatten)]
    pub common: SelectionCommon,
    pub concrete_type: Node<ScalarType>,
    pub is_custom_scalar: bool,
}
pub type SharedScalarSelection = Rc<ScalarSelection>;

impl ScalarSelection {
    pub fn new(
        common: SelectionCommon,
        concrete_type: Node<ScalarType>,
        is_custom_scalar: bool,
    ) -> SharedScalarSelection {
        Rc::new(ScalarSelection {
            common,
            concrete_type,
            is_custom_scalar,
        })
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct ObjectSelection {
    #[serde(flatten)]
    pub common: SelectionCommon,
    pub selections: RefCell<Vec<Selection>>,
}

pub type SharedObjectSelection = Rc<ObjectSelection>;

impl ObjectSelection {
    pub fn new(common: SelectionCommon) -> SharedObjectSelection {
        let ret = ObjectSelection {
            common,
            selections: RefCell::new(Vec::new()),
        };

        Rc::new(ret)
    }
    pub fn add_selection(&self, selection: Selection) {
        self.selections.borrow_mut().push(selection);
    }
}

#[derive(Debug, Serialize, Deserialize)]
pub struct EnumSelection {
    #[serde(flatten)]
    pub common: SelectionCommon,
    pub concrete_type: Node<EnumType>,
}

pub type SharedEnumSelection = Rc<EnumSelection>;

impl EnumSelection {
    pub fn new(common: SelectionCommon, concrete_type: Node<EnumType>) -> SharedEnumSelection {
        Rc::new(EnumSelection {
            common,
            concrete_type,
        })
    }
}

pub fn dart_type_for_scalar(scalar_name: &str) -> String {
    match scalar_name {
        "String" | "ID" => "String".to_string(),
        "Int" => "int".to_string(),
        "Float" => "double".to_string(),
        "Boolean" => "bool".to_string(),
        _ => "dynamic".to_string(),
    }
}
