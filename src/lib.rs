mod config;
mod log;
mod schemas;
mod utils;

use config::{ActionType, Config, JsConfig};
use log::error;
use valico::json_schema::ValidationState;
use std::fs;
use utils::set_panic_hook;
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
pub fn validate_action(src: &str, verbose: Option<bool>) -> JsValue {
    set_panic_hook();

    let config = JsConfig {
        action_type: ActionType::Action,
        src,
        verbose: verbose.unwrap_or(false),
    };

    run_js(&config)
}

#[wasm_bindgen(js_name = validateWorkflow)]
pub fn validate_workflow(src: &str, verbose: Option<bool>) -> JsValue {
    set_panic_hook();

    let config = JsConfig {
        action_type: ActionType::Workflow,
        src,
        verbose: verbose.unwrap_or(false),
    };

    run_js(&config)
}

pub fn run_js(config: &JsConfig) -> JsValue {
    let config = Config {
        file_name: None,
        action_type: config.action_type,
        src: config.src,
        verbose: config.verbose,
    };

    let result = run(&config);

    match result {
        Ok(state) => {
            serde_wasm_bindgen::to_value(&state).unwrap()
        }
        Err(e) => unimplemented!(),
    }
}

pub fn run_cli(config: &CliConfig) -> Result<(), Box<dyn std::error::Error>> {
    let file_name = config
        .src
        .file_name()
        .ok_or("Unable to derive file name from src!")?
        .to_str();

    let config = Config {
        file_name,
        action_type: match file_name {
            Some("action.yml") | Some("action.yaml") => ActionType::Action,
            _ => ActionType::Workflow,
        },
        src: &fs::read_to_string(&config.src)?,
        verbose: config.verbose,
    };

    let result = run(&config);

    match result {
        Ok(state) => {
            if !state.is_valid() {
                Err("validation failed".into())
            }
            else {
                Ok(())
            }
        }
        Err(e) => Err(e),
    }
}

fn run(config: &Config) -> Result<ValidationState, Box<dyn std::error::Error>> {
    let file_name = config.file_name.unwrap_or("file");
    let doc = serde_yaml::from_str(config.src)?;

    let state = match config.action_type {
        ActionType::Action => {
            if config.verbose {
                error(&format!("Treating {} as an Action definition", file_name));
            }
            validate_as_action(&doc)
        }
        ActionType::Workflow => {
            if config.verbose {
                error(&format!("Treating {} as a Workflow definition", file_name));
            }
            // TODO: Re-enable path and job validation
            validate_as_workflow(&doc) // && validate_paths(&doc) && validate_job_needs(&doc)
        }
    };

    Ok(state)
}

fn validate_paths(doc: &serde_json::Value) -> bool {
    let mut success = true;

    success = validate_globs(&doc["on"]["push"]["paths"], "on.push.paths") && success;
    success = validate_globs(&doc["on"]["push"]["paths-ignore"], "on.push.paths-ignore") && success;
    success =
        validate_globs(&doc["on"]["pull_request"]["paths"], "on.pull_request.paths") && success;
    success = validate_globs(
        &doc["on"]["pull_request"]["paths-ignore"],
        "on.pull_request.paths-ignore",
    ) && success;

    success
}

// TODO: Handle loading glob in WASM build
fn validate_globs(globs: &serde_json::Value, path: &str) -> bool {
    if globs.is_null() {
        true
    } else if let Some(globs) = globs.as_array() {
        let mut success = true;

        for g in globs {
            match glob(
                g.as_str().unwrap(),
            ) {
                Ok(res) => {
                    if res.count() == 0 {
                        error(&format!("Glob {g} in {path} does not match any files"));
                        success = false;
                    }
                }
                Err(e) => {
                    error(&format!("Glob {g} in {path} is invalid: {e}"));
                    success = false;
                }
            };
        }

        success
    } else {
        unreachable!("validate_globs called on globs object with invalid type: must be array or null")
    }
}

fn validate_job_needs(doc: &serde_json::Value) -> bool {
    fn is_invalid_dependency(jobs: &Map<String, Value>, need_str: &str) -> bool {
        !jobs.contains_key(need_str)
    }

    fn print_error(needs_str: &str) {
        error(&format!("unresolved job {needs_str}"));
    }

    let mut success = true;
    if let Some(jobs) = doc["jobs"].as_object() {
        for (_, job) in jobs.iter() {
            let needs = &job["needs"];
            if let Some(needs_str) = needs.as_str() {
                if is_invalid_dependency(jobs, needs_str) {
                    success = false;
                    print_error(needs_str);
                }
            } else if let Some(needs_array) = needs.as_array() {
                for needs_str in needs_array
                    .iter()
                    .filter_map(|v| v.as_str())
                    .filter(|needs_str| is_invalid_dependency(jobs, needs_str))
                {
                    success = false;
                    print_error(needs_str);
                }
            }
        }
    }

    success
}
