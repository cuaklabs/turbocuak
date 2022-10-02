use std::path;

use crate::common::domain::action::CommandHandlerFn;

use super::model::MonorepoState;

pub trait MonorepoStateProcessCommandHandler<TPathRef>: CommandHandlerFn<TPathRef, MonorepoState>
where TPathRef: AsRef<path::Path> {}

impl<T, TPathRef> MonorepoStateProcessCommandHandler<TPathRef> for T
where
  T: CommandHandlerFn<TPathRef, MonorepoState>,
  TPathRef: AsRef<path::Path> {}
