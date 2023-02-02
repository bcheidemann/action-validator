use clap::Parser;
use std::path::PathBuf;

#[derive(Parser, Debug)]
#[command(
    name = "action-validator",
    about = "A validator for GitHub Action and Workflow YAML files",
    version
)]
pub struct CliConfig {
    /// Be more verbose
    #[arg(short, long)]
    pub verbose: bool,

    /// Input file
    #[arg(name = "path_to_action_yaml")]
    pub src: PathBuf,
}

#[derive(Copy, Clone)]
pub enum ActionType {
    Action,
    Workflow,
}

pub struct JsConfig<'a> {
    pub action_type: ActionType,
    pub src: &'a str,
    pub verbose: bool,
}

pub struct Config<'a> {
    pub file_name: Option<&'a str>,
    pub action_type: ActionType,
    pub src: &'a str,
    pub verbose: bool,
}
