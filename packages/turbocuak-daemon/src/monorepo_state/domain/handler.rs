use std::path::PathBuf;

use crate::common::domain::action::CommandHandlerFn;

use super::model::MonorepoState;

pub trait MonorepoStateProcessCommandHandler: CommandHandlerFn<PathBuf, MonorepoState> {}

impl<T> MonorepoStateProcessCommandHandler for T where T: CommandHandlerFn<PathBuf, MonorepoState> {}
