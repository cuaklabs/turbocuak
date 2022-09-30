use std::path::Path;

use crate::common::domain::model::Result;
use crate::config::domain::model::{GlobalConfig, PackageConfig};

pub trait ParseGlobalConfigPortFn<TPath>: Fn(TPath) -> Result<GlobalConfig> where TPath: AsRef<Path> {}

impl<T, TPath>
  ParseGlobalConfigPortFn<TPath>
  for T
  where
    TPath: AsRef<Path>,
    T : Fn(TPath) -> Result<GlobalConfig> {}

pub trait ParsePackageConfigPortFn<TPath>: Fn(TPath) -> Result<PackageConfig> where TPath: AsRef<Path> {}

impl<T, TPath>
  ParsePackageConfigPortFn<TPath>
  for T
  where
    TPath: AsRef<Path>,
    T : Fn(TPath) -> Result<PackageConfig> {}
