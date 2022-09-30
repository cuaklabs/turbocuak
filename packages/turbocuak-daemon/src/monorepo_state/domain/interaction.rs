use std::collections;
use std::path;

use crate::common::domain::model::Result;
use crate::common::domain::action::{InteractionFn, BuildFn};
use crate::config::domain::model::{GlobalConfig, PackageConfig};
use crate::monorepo_state::domain::model::{MonorepoState, PackageState};

pub type MonorepoStateCreateCommand = (GlobalConfig, Vec<PackageConfig>, path::PathBuf);

fn create_monorepo_state_interaction(
  package_state_build: &impl BuildFn<PackageConfig, PackageState>
) -> impl InteractionFn<MonorepoStateCreateCommand, MonorepoState> + '_ {
  move |(global_config, package_configs, root_directory): MonorepoStateCreateCommand| -> Result<MonorepoState> {
    let package_state_results_iterator = package_configs.into_iter()
      .map(package_state_build);

    let mut package_states: Vec<PackageState> = vec![];

    for package_state_result in package_state_results_iterator {
      package_states.push(package_state_result?);
    }

    let package_path_to_state_map: collections::HashMap<path::PathBuf, PackageState> =
      package_states_to_package_path_to_state_map(package_states);

    let monorepo_state: MonorepoState = MonorepoState::new(global_config, package_path_to_state_map, root_directory);

    Ok(monorepo_state)
  }
}

fn package_states_to_package_path_to_state_map(package_states: Vec<PackageState>) -> collections::HashMap<path::PathBuf, PackageState> {
  collections::HashMap::from_iter(
    package_states
      .into_iter()
      .map(
        |package_state| -> (path::PathBuf, PackageState) {
          (path::PathBuf::from(&package_state.root), package_state)
        }
      )
  )
}
