use serde::Serialize;

use crate::validation_error::ValidationError;

#[derive(Serialize, Debug)]
pub struct ValidationState {
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
            errors: state.errors.iter().map(|err| err.into()).collect(),
        }
    }
}

impl From<&valico::json_schema::ValidationState> for ValidationState {
    fn from(state: &valico::json_schema::ValidationState) -> Self {
        ValidationState {
            errors: state.errors.iter().map(|err| err.into()).collect(),
        }
    }
}
