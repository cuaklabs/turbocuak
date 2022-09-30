use std::{collections::HashMap, path::PathBuf};

use crate::config::domain::model::GlobalConfig;

pub struct MonorepoState {
  pub global_config: GlobalConfig,
  pub package_path_to_state_map: HashMap<PathBuf, PackageState>,
  pub root_directory: PathBuf,
}

impl MonorepoState {
  pub fn new(
    global_config: GlobalConfig,
    package_path_to_state_map: HashMap<PathBuf, PackageState>,
    root_directory: PathBuf,
  ) -> Self {
    Self { global_config, package_path_to_state_map, root_directory }
  }
}

pub struct PackageState {
  pub name: String,
  pub root: PathBuf,
  pub tasks: Vec<TaskState>,
}

impl PackageState {
  pub fn new(
    name: String,
    root: PathBuf,
    tasks: Vec<TaskState>,
  ) -> Self {
    Self { name, root, tasks }
  }
}

pub struct TaskState {
  pub depends_on: Vec<String>,
  pub input_files: Vec<String>,
  pub is_synched: bool,
  pub name: String,
  pub stringified_options: String,
}

impl TaskState {
  pub fn new(
    depends_on: Vec<String>,
    input_files: Vec<String>,
    is_synched: bool,
    name: String,
    stringified_options: String,
  ) -> Self {
    Self {
      depends_on,
      input_files,
      is_synched,
      name,
      stringified_options,
    }
  }
}
