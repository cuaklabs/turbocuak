use std::path;

use crate::config::{
  domain::model::{GlobalConfig, TaskConfig, PackageConfig},
  infrastructure::serde_json::model::{GlobalConfigSerdeJson, PackageConfigSerdeJson, TaskConfigSerdeJson},
};

pub fn global_config_serde_json_to_global_config_conversion(global_config_serde_json: GlobalConfigSerdeJson) -> GlobalConfig {
  GlobalConfig::new(global_config_serde_json.package_directories)
}

pub fn package_config_serde_json_to_package_config_conversion(package_config_serde_json: PackageConfigSerdeJson) -> PackageConfig {
  PackageConfig::new(
    package_config_serde_json.name,
    path::PathBuf::from(&package_config_serde_json.root),
    package_config_serde_json.tasks
      .into_iter()
      .map(task_config_serde_json_to_task_config_conversion)
      .collect()
  )
}

pub fn task_config_serde_json_to_task_config_conversion(task_config_serde_json: TaskConfigSerdeJson) -> TaskConfig {
  TaskConfig::new(
    task_config_serde_json.depends_on,
    task_config_serde_json.input_files,
    task_config_serde_json.name,
    (*task_config_serde_json.stringified_options).to_string(),
  )
}
