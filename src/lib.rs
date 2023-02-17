mod config;
mod log;
mod schemas;
mod utils;
mod validation_error;
mod validation_state;

use config::{ActionType, JsConfig, RunConfig};
use std::fs;
use utils::set_panic_hook;
use validation_error::{ValidationError, ValidationErrorMetadata};
use validation_state::ValidationState;
use wasm_bindgen::prelude::*;

pub use crate::config::CliConfig;
use crate::schemas::{validate_as_action, validate_as_workflow};
use glob::glob;
use serde_json::{Map, Value};

// When the `wee_alloc` feature is enabled, use `wee_alloc` as the global
// allocator.
#[cfg(feature = "wee_alloc")]
#[global_allocator]
static ALLOC: wee_alloc::WeeAlloc = wee_alloc::WeeAlloc::INIT;

#[wasm_bindgen(js_name = validateAction)]
pub fn validate_action(src: &str) -> JsValue {
    set_panic_hook();

    let config = JsConfig {
        action_type: ActionType::Action,
        src,
        verbose: false,
    };

    run_js(&config)
}

#[wasm_bindgen(js_name = validateWorkflow)]
pub fn validate_workflow(src: &str) -> JsValue {
    set_panic_hook();

    let config = JsConfig {
        action_type: ActionType::Workflow,
        src,
        verbose: false,
    };

    run_js(&config)
}

pub fn run_js(config: &JsConfig) -> JsValue {
    let run_config = config.into();
    let state = run(&run_config);
    serde_wasm_bindgen::to_value(&state).unwrap()
}

pub fn run_cli(config: &CliConfig) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = config
        .src
        .file_name()
        .ok_or("Unable to derive file name from src!")?
        .to_str();

    let run_config = RunConfig {
        file_path: config.src.to_str(),
        file_name,
        action_type: match file_name {
            Some("action.yml") | Some("action.yaml") => ActionType::Action,
            _ => ActionType::Workflow,
        },
        src: &fs::read_to_string(&config.src)?,
        verbose: config.verbose,
    };

    let state = run(&run_config);

    if !state.is_valid() {
        log::error(&format!("Validation failed: {state:#?}"));
    }

    if state.is_valid() {
        Ok(())
    } else {
        Err("validation failed".into())
    }
}

fn run(config: &RunConfig) -> ValidationState {
    let file_name = config.file_name.unwrap_or("file");
    let doc = serde_yaml::from_str(config.src);

    let mut state = match doc {
        Err(err) => ValidationState {
            action_type: Some(config.action_type),
            file_path: Some(file_name.to_string()),
            errors: vec![err.into()],
        },
        Ok(doc) => match config.action_type {
            ActionType::Action => {
                if config.verbose {
                    log::log(&format!("Treating {} as an Action definition", file_name));
                }
                validate_as_action(&doc)
            }
            ActionType::Workflow => {
                if config.verbose {
                    log::log(&format!("Treating {} as a Workflow definition", file_name));
                }
                // TODO: Re-enable path and job validation
                let mut state = validate_as_workflow(&doc);

                validate_paths(&doc, &mut state);
                validate_job_needs(&doc, &mut state);

                state
            }
        },
    };

    state.action_type = Some(config.action_type);
    state.file_path = config.file_path.map(|file_name| file_name.to_string());

    state
}

fn validate_paths(doc: &serde_json::Value, state: &mut ValidationState) {
    validate_globs(&doc["on"]["push"]["paths"], "/on/push/paths", state);
    validate_globs(
        &doc["on"]["push"]["paths-ignore"],
        "/on/push/paths-ignore",
        state,
    );
    validate_globs(
        &doc["on"]["pull_request"]["paths"],
        "/on/pull_request/paths",
        state,
    );
    validate_globs(
        &doc["on"]["pull_request"]["paths-ignore"],
        "/on/pull_request/paths-ignore",
        state,
    );
}

#[cfg(feature = "js")]
fn validate_globs(value: &serde_json::Value, path: &str, _: &mut ValidationState) {
    if !value.is_null() {
        log::warn(&format!(
            "WARNING: Glob validation is not yet supported. Glob at {path} will not be validated."
        ));
    }
}

#[cfg(not(feature = "js"))]
fn validate_globs(globs: &serde_json::Value, path: &str, state: &mut ValidationState) {
    if globs.is_null() {
        return;
    }

    if let Some(globs) = globs.as_array() {
        for g in globs {
            match glob(g.as_str().unwrap()) {
                Ok(res) => {
                    if res.count() == 0 {
                        state
                            .errors
                            .push(ValidationError::NoFilesMatchingGlobError {
                                meta: ValidationErrorMetadata {
                                    code: "glob_not_matched".into(),
                                    path: path.into(),
                                    title: "Glob does not match any files".into(),
                                    detail: Some(format!(
                                        "Glob {g} in {path} does not match any files"
                                    )),
                                },
                            });
                    }
                }
                Err(e) => {
                    state.errors.push(ValidationError::InvalidGlobError {
                        meta: ValidationErrorMetadata {
                            code: "invalid_glob".into(),
                            path: path.into(),
                            title: "Glob does not match any files".into(),
                            detail: Some(format!("Glob {g} in {path} is invalid: {e}")),
                        },
                    });
                }
            };
        }
    } else {
        unreachable!(
            "validate_globs called on globs object with invalid type: must be array or null"
        )
    }
}

fn validate_job_needs(doc: &serde_json::Value, state: &mut ValidationState) {
    fn is_invalid_dependency(jobs: &Map<String, Value>, need_str: &str) -> bool {
        !jobs.contains_key(need_str)
    }

    fn handle_unresolved_job(job_name: &String, needs_str: &str, state: &mut ValidationState) {
        state.errors.push(ValidationError::UnresolvedJobError {
            meta: ValidationErrorMetadata {
                code: "unresolved_job".into(),
                path: format!("/jobs/{job_name}/needs"),
                title: "Unresolved job".into(),
                detail: Some(format!("unresolved job {needs_str}")),
            },
        });
    }

    if let Some(jobs) = doc["jobs"].as_object() {
        for (job_name, job) in jobs.iter() {
            let needs = &job["needs"];
            if let Some(needs_str) = needs.as_str() {
                if is_invalid_dependency(jobs, needs_str) {
                    handle_unresolved_job(job_name, needs_str, state);
                }
            } else if let Some(needs_array) = needs.as_array() {
                for needs_str in needs_array
                    .iter()
                    .filter_map(|v| v.as_str())
                    .filter(|needs_str| is_invalid_dependency(jobs, needs_str))
                {
                    handle_unresolved_job(job_name, needs_str, state);
                }
            }
        }
    }
}
