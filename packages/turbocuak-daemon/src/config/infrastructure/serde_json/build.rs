use crate::common::domain::model::Result;
use crate::common::domain::action::{BuildFn, ConversionFn};
use crate::config::domain::model::{
  GlobalConfig,
  PackageConfig,
};
use crate::config::infrastructure::serde_json::conversion::{
  global_config_serde_json_to_global_config_conversion,
  package_config_serde_json_to_package_config_conversion,
};
use crate::config::infrastructure::serde_json::model::{
  GlobalConfigSerdeJson,
  PackageConfigSerdeJson,
};

fn global_config_build_generator(
  global_config_serde_json_to_global_config_conversion: &impl ConversionFn<GlobalConfigSerdeJson, GlobalConfig>
) -> impl BuildFn<String, GlobalConfig> + '_ {
  move |stringified_global_config: String| -> Result<GlobalConfig> {
    let global_config_serde_json: GlobalConfigSerdeJson = serde_json::from_str(stringified_global_config.as_ref())?;
    let global_config: GlobalConfig = global_config_serde_json_to_global_config_conversion(global_config_serde_json);

    Ok(global_config)
  }
}

pub fn global_config_build(stringified_global_config: String) -> Result<GlobalConfig> {
  global_config_build_generator(&global_config_serde_json_to_global_config_conversion)(stringified_global_config)
}

fn package_config_build_generator(
  package_config_serde_json_to_package_config_conversion: &impl ConversionFn<PackageConfigSerdeJson, PackageConfig>
) -> impl BuildFn<String, PackageConfig> + '_ {
  move |stringified_package_config: String| -> Result<PackageConfig> {
    let package_config_serde_json: PackageConfigSerdeJson = serde_json::from_str(&stringified_package_config)?;
    let package_config: PackageConfig = package_config_serde_json_to_package_config_conversion(package_config_serde_json);

    Ok(package_config)
  }
}

pub fn package_config_build(stringified_task_config: String) -> Result<PackageConfig> {
  package_config_build_generator(&package_config_serde_json_to_package_config_conversion)(stringified_task_config)
}

#[cfg(test)]
mod tests {

  mod global_config_build_tests {
    use crate::common::domain::model::Result;
    use crate::config::domain::model::GlobalConfig;
    use crate::config::infrastructure::serde_json::build::global_config_build;

    #[test]
    fn it_parses_valid_config() -> Result<()> {
      let data: String = String::from(r#"{
  "packageDirectories": ["package"]
}"#);

      let result: GlobalConfig = global_config_build(data)?;
      let expected_result: GlobalConfig = GlobalConfig::new(vec![String::from("package")]);

      assert_eq!(
        result,
        expected_result
      );

      Ok(())
    }

    #[test]
    fn it_parses_valid_config_with_additional_properties() -> Result<()> {
      let data: String = String::from(r#"{
  "foo": "bar",
  "packageDirectories": ["package"]
}"#);

      let result: GlobalConfig = global_config_build(data)?;
      let expected_result: GlobalConfig = GlobalConfig::new(vec![String::from("package")]);

      assert_eq!(
        result,
        expected_result
      );

      Ok(())
    }

    #[test]
    fn it_does_not_parse_config_with_invalid_properties() -> Result<()> {
      let data: String = String::from(r#"{
  "foo": "bar",
  "packageDirectories": [3]
}"#);

      let result = global_config_build(data);

      assert!(result.is_err());

      Ok(())
    }
  }

  mod package_config_build_tests {
    use std::path::PathBuf;

    use crate::common::domain::model::Result;
    use crate::config::domain::model::{PackageConfig, TaskConfig};
    use crate::config::infrastructure::serde_json::build::package_config_build;

    #[test]
    fn it_parses_valid_config() -> Result<()> {
      let data: String = String::from(r#"{
  "name": "name",
  "root": "root",
  "tasks": [
    {
      "depends_on": [],
      "inputFiles": [],
      "name": "taskName",
      "options": { "foo": "bar" }
    }
  ]
}"#);

      let result: PackageConfig = package_config_build(data)?;
      let expected_result: PackageConfig = PackageConfig::new(
        String::from("name"),
        PathBuf::from("root"),
        vec![
          TaskConfig::new(vec![], vec![], String::from("taskName"), String::from("{ \"foo\": \"bar\" }"))
        ]
      );

      assert_eq!(
        result,
        expected_result
      );

      Ok(())
    }
  }
}
