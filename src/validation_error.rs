use serde::Serialize;
use valico::common::error::ValicoError;

use crate::validation_state::ValidationState;

type BoxedValicoError = Box<dyn ValicoError>;

#[derive(Serialize, Debug)]
pub struct ValidationErrorMetadata {
    pub code: String,
    pub path: String,
    pub title: String,
    pub detail: Option<String>,
}

impl ValidationErrorMetadata {
    pub fn from_valico_error(err: &impl ValicoError) -> Self {
        ValidationErrorMetadata {
            code: err.get_code().into(),
            path: err.get_path().into(),
            title: err.get_title().into(),
            detail: err.get_detail().map(|s| s.into()),
        }
    }
}

impl From<&BoxedValicoError> for ValidationErrorMetadata {
    fn from(err: &BoxedValicoError) -> Self {
        ValidationErrorMetadata {
            code: err.get_code().into(),
            path: err.get_path().into(),
            title: err.get_title().into(),
            detail: err.get_detail().map(|s| s.into()),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ParseErrorLocation {
    pub index: usize,
    pub line: usize,
    pub column: usize,
}

impl From<serde_yaml::Location> for ParseErrorLocation {
    fn from(location: serde_yaml::Location) -> Self {
        ParseErrorLocation {
            index: location.index(),
            line: location.line(),
            column: location.column(),
        }
    }
}

#[derive(Serialize, Debug)]
pub struct ParseErrorMetadata {
    pub code: String,
    pub title: String,
    pub detail: String,
    pub location: Option<ParseErrorLocation>,
}

impl From<serde_yaml::Error> for ParseErrorMetadata {
    fn from(err: serde_yaml::Error) -> Self {
        ParseErrorMetadata {
            code: "parse_error".into(),
            title: "Parse Error".into(),
            detail: err.to_string(),
            location: err.location().map(|location| location.into()),
        }
    }
}

#[derive(Serialize, Debug)]
#[serde(untagged)]
pub enum ValidationError {
    // Schema Validation Errors
    WrongTypeSchemaError {
        meta: ValidationErrorMetadata,
    },
    MultipleOfSchemaError {
        meta: ValidationErrorMetadata,
    },
    MaximumSchemaError {
        meta: ValidationErrorMetadata,
    },
    MinimumSchemaError {
        meta: ValidationErrorMetadata,
    },
    MaxLengthSchemaError {
        meta: ValidationErrorMetadata,
    },
    MinLengthSchemaError {
        meta: ValidationErrorMetadata,
    },
    PatternSchemaError {
        meta: ValidationErrorMetadata,
    },
    MaxItemsSchemaError {
        meta: ValidationErrorMetadata,
    },
    MinItemsSchemaError {
        meta: ValidationErrorMetadata,
    },
    UniqueItemsSchemaError {
        meta: ValidationErrorMetadata,
    },
    ItemsSchemaError {
        meta: ValidationErrorMetadata,
    },
    MaxPropertiesSchemaError {
        meta: ValidationErrorMetadata,
    },
    MinPropertiesSchemaError {
        meta: ValidationErrorMetadata,
    },
    RequiredSchemaError {
        meta: ValidationErrorMetadata,
    },
    PropertiesSchemaError {
        meta: ValidationErrorMetadata,
    },
    EnumSchemaError {
        meta: ValidationErrorMetadata,
    },
    AnyOfSchemaError {
        states: Vec<ValidationState>,
        meta: ValidationErrorMetadata,
    },
    OneOfSchemaError {
        states: Vec<ValidationState>,
        meta: ValidationErrorMetadata,
    },
    ConstSchemaError {
        meta: ValidationErrorMetadata,
    },
    ContainsSchemaError {
        meta: ValidationErrorMetadata,
    },
    ContainsMinMaxSchemaError {
        meta: ValidationErrorMetadata,
    },
    NotSchemaError {
        meta: ValidationErrorMetadata,
    },
    DivergentDefaultsSchemaError {
        meta: ValidationErrorMetadata,
    },
    FormatSchemaError {
        meta: ValidationErrorMetadata,
    },
    UnevaluatedSchemaError {
        meta: ValidationErrorMetadata,
    },
    UnknownSchemaError {
        meta: ValidationErrorMetadata,
    },

    // Other Validation Errors
    UnresolvedJobError {
        meta: ValidationErrorMetadata,
    },
    InvalidGlobError {
        meta: ValidationErrorMetadata,
    },
    NoFilesMatchingGlobError {
        meta: ValidationErrorMetadata,
    },

    // Other Errors
    ParseError {
        meta: ParseErrorMetadata,
    },
}

impl From<&BoxedValicoError> for ValidationError {
    fn from(err: &BoxedValicoError) -> Self {
        if let Some(err) = err.downcast::<valico::json_schema::errors::WrongType>() {
            ValidationError::WrongTypeSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::MultipleOf>() {
            ValidationError::MultipleOfSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::Maximum>() {
            ValidationError::MaximumSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::Minimum>() {
            ValidationError::MinimumSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::MaxLength>() {
            ValidationError::MaxLengthSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::MinLength>() {
            ValidationError::MinLengthSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::Pattern>() {
            ValidationError::PatternSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::MaxItems>() {
            ValidationError::MaxItemsSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::MinItems>() {
            ValidationError::MinItemsSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::UniqueItems>() {
            ValidationError::UniqueItemsSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::Items>() {
            ValidationError::ItemsSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::MaxProperties>() {
            ValidationError::MaxPropertiesSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::MinProperties>() {
            ValidationError::MinPropertiesSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::Required>() {
            ValidationError::RequiredSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::Properties>() {
            ValidationError::PropertiesSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::Enum>() {
            ValidationError::EnumSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::AnyOf>() {
            ValidationError::AnyOfSchemaError {
                states: err
                    .states
                    .iter()
                    .map(|state| ValidationState::from(state))
                    .collect(),
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::OneOf>() {
            ValidationError::OneOfSchemaError {
                states: err
                    .states
                    .iter()
                    .map(|state| ValidationState::from(state))
                    .collect(),
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::Const>() {
            ValidationError::ConstSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::Contains>() {
            ValidationError::ContainsSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::ContainsMinMax>() {
            ValidationError::ContainsMinMaxSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::Not>() {
            ValidationError::NotSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::DivergentDefaults>() {
            ValidationError::DivergentDefaultsSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::Format>() {
            ValidationError::FormatSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else if let Some(err) = err.downcast::<valico::json_schema::errors::Unevaluated>() {
            ValidationError::UnevaluatedSchemaError {
                meta: ValidationErrorMetadata::from_valico_error(err),
            }
        } else {
            ValidationError::UnknownSchemaError { meta: err.into() }
        }
    }
}

impl From<serde_yaml::Error> for ValidationError {
    fn from(err: serde_yaml::Error) -> Self {
        ValidationError::ParseError { meta: err.into() }
    }
}
