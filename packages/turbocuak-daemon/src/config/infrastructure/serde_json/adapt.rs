use std::fs;
use std::path;

use crate::common::domain::action::BuildFn;
use crate::common::domain::model::Error;
use crate::common::domain::model::Result;
use crate::config::domain::{GLOBAL_CONFIG_FILE_NAME, PACKAGE_CONFIG_FILE_NAME};
use crate::config::domain::model::{GlobalConfig, PackageConfig};
use crate::config::domain::port::{ParseGlobalConfigPortFn, ParsePackageConfigPortFn};
use crate::config::infrastructure::serde_json::build::{global_config_build, package_config_build};

fn parse_global_config_adapt_generator<TPath: AsRef<path::Path>>(
  global_config_build: impl BuildFn<String, GlobalConfig>
) -> impl ParseGlobalConfigPortFn<TPath> {
  move |root_path: TPath| -> Result<GlobalConfig> {
    let root_canonical_path: path::PathBuf = fs::canonicalize(root_path)?;
    let global_config_path: path::PathBuf = root_canonical_path.join(GLOBAL_CONFIG_FILE_NAME);

    if global_config_path.is_file() {
      let global_config_raw: String = fs::read_to_string(global_config_path)?;
      let global_config: GlobalConfig = global_config_build(global_config_raw)?;

      Ok(global_config)
    } else {
      let error_message: String = format!("Expecting a file at {}", global_config_path.to_str().unwrap());

      return Err(Error::new(error_message))
    }
  }
}

pub fn parse_global_config_adapt<TPath: AsRef<path::Path>>(root_path: TPath) -> Result<GlobalConfig> {
  parse_global_config_adapt_generator(global_config_build)(root_path)
}

fn parse_package_config_adapt_generator<TPath: AsRef<path::Path>>(
  package_config_build: impl BuildFn<String, PackageConfig>
) -> impl ParsePackageConfigPortFn<TPath> {
  move |root_path: TPath| -> Result<PackageConfig> {
    let root_canonical_path: path::PathBuf = fs::canonicalize(root_path)?;
    let package_config_path: path::PathBuf = root_canonical_path.join(PACKAGE_CONFIG_FILE_NAME);

    if package_config_path.is_file() {
      let package_config_raw: String = fs::read_to_string(package_config_path)?;
      let package_config: PackageConfig = package_config_build(package_config_raw)?;

      Ok(package_config)
    } else {
      let error_message: String = format!("Expecting a file at {}", package_config_path.to_str().unwrap());

      return Err(Error::new(error_message))
    }
  }
}

pub fn parse_package_config_adapt<TPath: AsRef<path::Path>>(root_path: TPath) -> Result<PackageConfig> {
  parse_package_config_adapt_generator(package_config_build)(root_path)
}
