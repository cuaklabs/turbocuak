use std::{path, sync};

use crate::common::domain::action::InteractionFn;
use crate::common::domain::model::Result;
use crate::config::domain::model::GlobalConfig;
use crate::config::domain::port::ParseGlobalConfigPortFn;
use crate::config::infrastructure::serde_json::adapt::parse_global_config_adapt;
use crate::monorepo_state::domain::handler::{
  MonorepoStateProcessCommandHandler,
  MonorepoStateProcessCommandHandlerResult,
};
use crate::monorepo_state::domain::interaction::{
  MonorepoStateCreateCommand,
  create_monorepo_state_interaction,
};
use crate::monorepo_state::domain::model::MonorepoState;

pub fn monorepo_state_process_command_handler(path: &path::Path) -> Result<MonorepoStateProcessCommandHandlerResult> {
  monorepo_state_process_command_handler_generator(
    &create_monorepo_state_interaction,
    &parse_global_config_adapt,
  )(path)
}

fn monorepo_state_process_command_handler_generator<'actions, 'path>(
  create_monorepo_state_interaction: &'actions impl InteractionFn<MonorepoStateCreateCommand, MonorepoState>,
  parse_global_config_port: &'actions impl ParseGlobalConfigPortFn<&'path path::Path>,
) -> impl MonorepoStateProcessCommandHandler<'actions, 'path> {
  move |path: &'path path::Path| -> Result<MonorepoStateProcessCommandHandlerResult<'actions>> {
    let monorepo_state_result: Result<MonorepoState> = monorepo_state_process_command_handler_generator_parse_state(
      create_monorepo_state_interaction,
      parse_global_config_port,
      path,
    );

    let monorepo_state_option: Option<MonorepoState> = 
      match monorepo_state_result {
        Ok(monorepo_state) => Some(monorepo_state),
        Err(_) => None
      };

    Ok(
      MonorepoStateProcessCommandHandlerResult::new(
        sync::Arc::new(sync::Mutex::new(monorepo_state_option)),
        Box::pin(foo()),
      )
    )
  }
}

async fn foo() -> () {}

fn monorepo_state_process_command_handler_generator_parse_state<'a, 'b>(
  create_monorepo_state_interaction: &'a impl InteractionFn<MonorepoStateCreateCommand, MonorepoState>,
  parse_global_config_port: &'a impl ParseGlobalConfigPortFn<&'b path::Path>,
  path: &'b path::Path,
) -> Result<MonorepoState> {
  let global_config: GlobalConfig = parse_global_config_port(path.as_ref())?;

  let monorepo_state_create_command: MonorepoStateCreateCommand = (global_config, vec![], path::PathBuf::from(path));
  let monorepo_state: MonorepoState = create_monorepo_state_interaction(monorepo_state_create_command)?;

  Ok(monorepo_state)
}
