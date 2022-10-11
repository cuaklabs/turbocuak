use std::collections;
use std::path;

use crate::common::domain::model::Result;
use crate::common::domain::action::{InteractionFn, BuildFn};
use crate::config::domain::model::{GlobalConfig, PackageConfig};
use crate::config::domain::port::ParseGlobalConfigPortFn;
use crate::config::infrastructure::serde_json::adapt::parse_global_config_adapt;
use crate::monorepo_state::domain::model::{MonorepoState, PackageState};
use crate::monorepo_state::domain::build::package_state_build;

pub type MonorepoStateCreateCommand = (GlobalConfig, Vec<PackageConfig>, path::PathBuf);

pub fn create_monorepo_state_interaction((global_config, package_configs, root_directory): MonorepoStateCreateCommand) -> Result<MonorepoState> {
  create_monorepo_state_interaction_generator(&package_state_build)(
    (global_config, package_configs, root_directory)
  )
}

pub fn parse_monorepo_state_interaction(path: &path::Path) -> Result<MonorepoState> {
  parse_monorepo_state_interaction_generator(
    &create_monorepo_state_interaction,
    &parse_global_config_adapt,
  )(path)
}

fn create_monorepo_state_interaction_generator(
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

    let monorepo_state: MonorepoState = MonorepoState::new(
      global_config,
      package_path_to_state_map,
      path::PathBuf::from(root_directory),
    );

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

fn parse_monorepo_state_interaction_generator<'path>(
  create_monorepo_state_interaction: &'path impl InteractionFn<MonorepoStateCreateCommand, MonorepoState>,
  parse_global_config_port: &'path impl ParseGlobalConfigPortFn<&'path path::Path>,
) -> impl InteractionFn<&'path path::Path, MonorepoState> + 'path {
  move |path: &'path path::Path| -> Result<MonorepoState> {
    let global_config: GlobalConfig = parse_global_config_port(path)?;
    let monorepo_state_create_command: MonorepoStateCreateCommand = (
      global_config,
      vec![],
      path::PathBuf::from(path)
    );

    create_monorepo_state_interaction(monorepo_state_create_command)
  }
}
