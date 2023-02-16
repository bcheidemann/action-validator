use serde::Serialize;

use crate::{validation_error::ValidationError, config::ActionType};

#[derive(Serialize, Debug)]
pub struct ValidationState {
    #[serde(rename = "actionType")]
    pub action_type: Option<ActionType>,
    #[serde(rename = "filePath")]
    pub file_path: Option<String>,
    pub errors: Vec<ValidationError>,
}

impl ValidationState {
    pub fn is_valid(&self) -> bool {
        self.errors.is_empty()
    }
}

impl From<valico::json_schema::ValidationState> for ValidationState {
    fn from(state: valico::json_schema::ValidationState) -> Self {
        ValidationState {
            file_path: None,
            action_type: None,
            errors: state.errors.iter().map(|err| err.into()).collect(),
        }
    }
}

impl From<&valico::json_schema::ValidationState> for ValidationState {
    fn from(state: &valico::json_schema::ValidationState) -> Self {
        ValidationState {
            file_path: None,
            action_type: None,
            errors: state.errors.iter().map(|err| err.into()).collect(),
        }
    }
}
