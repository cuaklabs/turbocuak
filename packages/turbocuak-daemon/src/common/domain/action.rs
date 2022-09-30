use crate::common::domain::model::Result;

pub trait BuildFn<TParams, TModel>: Fn(TParams) -> Result<TModel> {}

impl<T, TParams, TModel>
  BuildFn<TParams, TModel>
  for T
  where T : Fn(TParams) -> Result<TModel> {}

pub trait CommandHandlerFn<TCommand, TResult>: Fn(TCommand) -> Result<TResult> {}

impl<T, TParams, TResult>
  CommandHandlerFn<TParams, TResult>
  for T
  where T : Fn(TParams) -> Result<TResult> {}

pub trait ConversionFn<TInput, TOutput>: Fn(TInput) -> TOutput {}

impl<T, TInput, TOutput>
  ConversionFn<TInput, TOutput>
  for T
  where T : Fn(TInput) -> TOutput {}

pub trait InteractionFn<TParams, TOutput>: Fn(TParams) -> Result<TOutput>{}

impl<T, TOutput, TModel>
  InteractionFn<TOutput, TModel>
  for T
  where T : Fn(TOutput) -> Result<TModel> {}
