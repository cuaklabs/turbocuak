use async_trait::async_trait;

use crate::common::domain::model::Result;

#[async_trait]
pub trait WatchFileSystenPortStopwatch {
  async fn unwatch(&mut self) -> Result<()>;
}

#[async_trait]
pub trait WatchFileSystemPort {
  async fn watch(&mut self) -> Result<Box<dyn WatchFileSystenPortStopwatch>>;
}
