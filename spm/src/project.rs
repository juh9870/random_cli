use crate::ide_runner::IdeRunner;
use serde::{Deserialize, Serialize};
use std::path::PathBuf;

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Project {
    pub path: PathBuf,
    pub name: String,
    pub runner: Option<IdeRunner>,
}

impl Project {
    pub fn run(&self) {}
}
