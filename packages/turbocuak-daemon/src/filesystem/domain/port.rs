use async_trait::async_trait;

use crate::common::domain::model::Result;

#[async_trait]
pub trait WatchFileSystenPortWatch {
  async fn watch(&mut self) -> Result<()>;
}

#[async_trait]
pub trait WatchFileSystenPortStopwatch {
  async fn unwatch(&mut self) -> Result<()>;
}

pub trait WatchFileSystemPort {
  fn prepare(&mut self) -> Result<
    (Box<dyn WatchFileSystenPortWatch + '_>, Box<dyn WatchFileSystenPortStopwatch + '_>)
  >;
}
