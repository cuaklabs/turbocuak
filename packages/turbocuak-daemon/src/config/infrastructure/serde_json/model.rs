use serde::{Serialize, Deserialize};
use serde_json::value::RawValue;

#[derive(Serialize, Deserialize)]
pub struct GlobalConfigSerdeJson {
  #[serde(rename = "packageDirectories")]
  pub package_directories: Vec<String>,
}

#[derive(Serialize, Deserialize)]
pub struct PackageConfigSerdeJson {
  pub name: String,
  pub root: String,
  pub tasks: Vec<TaskConfigSerdeJson>,
}

#[derive(Serialize, Deserialize)]
pub struct TaskConfigSerdeJson {
  #[serde(rename = "depends_on")]
  pub depends_on: Vec<String>,
  #[serde(rename = "inputFiles")]
  pub input_files: Vec<String>,
  pub name: String,
  #[serde(rename = "options")]
  pub stringified_options: Box<RawValue>,
}
