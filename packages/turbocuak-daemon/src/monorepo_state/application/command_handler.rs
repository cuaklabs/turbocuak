use std::path;

use crate::common::domain::action::InteractionFn;
use crate::common::domain::model::Result;
use crate::config::domain::model::GlobalConfig;
use crate::config::domain::port::ParseGlobalConfigPortFn;
use crate::config::infrastructure::serde_json::adapt::parse_global_config_adapt;
use crate::monorepo_state::domain::handler::MonorepoStateProcessCommandHandler;
use crate::monorepo_state::domain::interaction::{MonorepoStateCreateCommand, create_monorepo_state_interaction};
use crate::monorepo_state::domain::model::MonorepoState;

pub fn monorepo_state_process_command_handler<TPathRef: AsRef<path::Path>>(path: TPathRef) -> Result<MonorepoState> {
  monorepo_state_process_command_handler_generator(
    &create_monorepo_state_interaction,
    &parse_global_config_adapt,
  )(path)
}

fn monorepo_state_process_command_handler_generator<'a, TPathRef: AsRef<path::Path>>(
  create_monorepo_state_interaction: &'a impl InteractionFn<MonorepoStateCreateCommand, MonorepoState>,
  parse_global_config_port: &'a impl ParseGlobalConfigPortFn<TPathRef>,
) -> impl MonorepoStateProcessCommandHandler<TPathRef> + 'a {
  move |path: TPathRef| -> Result<MonorepoState> {
    let root_directory: path::PathBuf = path::PathBuf::from(path.as_ref());
    let global_config: GlobalConfig = parse_global_config_port(path)?;

    let monorepo_state_create_command: MonorepoStateCreateCommand = (global_config, vec![], root_directory);
    let monorepo_state: MonorepoState = create_monorepo_state_interaction(monorepo_state_create_command)?;

    Ok(monorepo_state)
  }
}

