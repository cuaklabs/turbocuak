use std::path;

#[derive(Debug, PartialEq)]
pub struct GlobalConfig {
  pub package_directories: Vec<String>,
}

impl GlobalConfig {
  pub fn new(package_directories: Vec<String>) -> Self {
    Self { package_directories }
  }
}

#[derive(Debug, PartialEq)]
pub struct PackageConfig {
  pub name: String,
  pub root: path::PathBuf,
  pub tasks: Vec<TaskConfig>,
}

impl PackageConfig {
  pub fn new(
    name: String,
    root: path::PathBuf,
    tasks: Vec<TaskConfig>,
  ) -> Self {
    Self { name, root, tasks }
  }
}

#[derive(Debug, PartialEq)]
pub struct TaskConfig {
  pub depends_on: Vec<String>,
  pub input_files: Vec<String>,
  pub name: String,
  pub stringified_options: String,
}

impl TaskConfig {
  pub fn new(
    depends_on: Vec<String>,
    input_files: Vec<String>,
    name: String,
    stringified_options: String,
  ) -> Self {
    Self { depends_on, input_files, name, stringified_options }
  }
}
