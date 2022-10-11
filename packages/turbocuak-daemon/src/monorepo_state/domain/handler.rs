use std::{path, sync, pin};

use crate::common::domain::action::CommandHandlerFn;
use crate::monorepo_state::domain::model::MonorepoState;

pub struct MonorepoStateProcessCommandHandlerResult<'a> {
  pub monorepo_state: sync::Arc<sync::Mutex<Option<MonorepoState>>>,
  pub background_process: pin::Pin<Box<dyn futures::Future<Output = ()> + Send + 'a>>,
}

impl <'a> MonorepoStateProcessCommandHandlerResult<'a> {
  pub fn new(
    monorepo_state: sync::Arc<sync::Mutex<Option<MonorepoState>>>,
    background_process: pin::Pin<Box<dyn futures::Future<Output = ()> + Send + 'a>>,
  ) -> Self {
    Self { monorepo_state, background_process }
  }
}

pub trait MonorepoStateProcessCommandHandler<'a, 'path>: CommandHandlerFn<&'path path::Path, MonorepoStateProcessCommandHandlerResult<'a>>
{}

impl<'a, 'path, T> MonorepoStateProcessCommandHandler<'a, 'path> for T
where
  T: CommandHandlerFn<&'path path::Path, MonorepoStateProcessCommandHandlerResult<'a>> {}
