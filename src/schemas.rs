use serde_json::Value;

use crate::log::error;

pub fn validate_as_action(doc: &Value) -> bool {
    validate_with_schema(
        doc,
        include_bytes!("schemastore/src/schemas/json/github-action.json"),
    )
}

pub fn validate_as_workflow(doc: &Value) -> bool {
    validate_with_schema(
        doc,
        include_bytes!("schemastore/src/schemas/json/github-workflow.json"),
    )
}

fn validate_with_schema(doc: &Value, schema: &[u8]) -> bool {
    let schema_json: serde_json::Value =
        serde_json::from_str(String::from_utf8_lossy(schema).as_ref()).unwrap();

    let mut scope = valico::json_schema::Scope::new();
    let validator = scope.compile_and_return(schema_json, false).unwrap();

    let state = validator.validate(doc);

    if state.is_valid() {
        true
    } else {
        error(&format!("Validation failed: {state:#?}"));
        false
    }
}
