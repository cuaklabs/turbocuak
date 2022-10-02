use std::path::Path;

use crate::common::domain::model::Result;
use crate::config::domain::model::{GlobalConfig, PackageConfig};

pub trait ParseGlobalConfigPortFn<TPathRef>: Fn(TPathRef) -> Result<GlobalConfig> where TPathRef: AsRef<Path> {}

impl<T, TPathRef>
  ParseGlobalConfigPortFn<TPathRef>
  for T
  where
    TPathRef: AsRef<Path>,
    T : Fn(TPathRef) -> Result<GlobalConfig> {}

pub trait ParsePackageConfigPortFn<TPathRef>: Fn(TPathRef) -> Result<PackageConfig> where TPathRef: AsRef<Path> {}

impl<T, TPathRef>
  ParsePackageConfigPortFn<TPathRef>
  for T
  where
    TPathRef: AsRef<Path>,
    T : Fn(TPathRef) -> Result<PackageConfig> {}
