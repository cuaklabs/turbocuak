use crate::common::domain::model::Result;
use crate::config::domain::model::{TaskConfig, PackageConfig};
use crate::monorepo_state::domain::model::{PackageState, TaskState};

pub fn package_state_build(package_config: PackageConfig) -> Result<PackageState> {
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
