use std::collections::HashMap;
use std::fs;
use std::path::{Path, PathBuf};

use crate::common::domain::model::Error;
use crate::config::domain::model::GlobalConfig;
use crate::config::infrastructure::serde_json::adapt::parse_global_config_adapt;
use crate::monorepo_state::domain::model::{MonorepoState, PackageState};
use crate::monorepo_state::package_state_build::package_state_build;

pub fn monorepo_state_build<T: AsRef<Path>>(root_path: T) -> Result<MonorepoState, Error> {
  let root_canonical_path: PathBuf = fs::canonicalize(root_path)?;

  let global_config: GlobalConfig = parse_global_config_adapt(&root_canonical_path)?;

  let package_states: Vec<PackageState> = (&global_config.package_directories)
    .iter()
    .map(package_state_build)
    .filter(Result::is_ok)
    .map(Result::unwrap)
    .collect();

  let package_path_to_state_map: HashMap<PathBuf, PackageState> = HashMap::from_iter(
    package_states
      .into_iter()
      .map(
        |package_state| -> (PathBuf, PackageState) {
          (PathBuf::from(&package_state.root), package_state)
        }
      )
  );

  let monorepo_state: MonorepoState = MonorepoState::new(
    global_config,
    package_path_to_state_map,
    root_canonical_path,
  );

  return Ok(monorepo_state)
}
