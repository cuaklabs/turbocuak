use std::path;

use crate::common::domain::model::Error;
use crate::config::domain::model::{PackageConfig, TaskConfig};
use crate::config::infrastructure::serde_json::adapt::parse_package_config_adapt;
use crate::monorepo_state::domain::model::{PackageState, TaskState};

pub fn package_state_build<T: AsRef<path::Path>>(directory_path: T) -> Result<PackageState, Error> {
  let package_config: PackageConfig = parse_package_config_adapt(directory_path)?;

  Ok(
    PackageState::new(
      package_config.name,
      package_config.root,
      package_config.tasks.into_iter().map(task_config_to_task_state).collect()
    )
  )
}

fn task_config_to_task_state(task_config: TaskConfig) -> TaskState {
  TaskState::new(
    task_config.depends_on,
    task_config.input_files,
    false,
    task_config.name,
    task_config.stringified_options
  )
}
