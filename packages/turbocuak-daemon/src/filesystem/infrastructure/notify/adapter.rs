use std::path;

use async_trait::async_trait;
use core::result;
use futures::channel::mpsc::{channel, Receiver, Sender};
use futures::{SinkExt, StreamExt};
use notify::{
  Config,
  Error as NotifyError,
  Event,
  RecommendedWatcher,
  Watcher,
  RecursiveMode,
};
use notify::inotify::INotifyWatcher;

use crate::common::domain::model::Result;
use crate::filesystem::domain::port::{WatchFileSystemPort, WatchFileSystenPortStopwatch};

pub trait WatchFileSystemErrCallback: Fn(NotifyError) -> () {}

impl<T>
  WatchFileSystemErrCallback
  for T
  where T : Fn(NotifyError) -> () {}

pub trait WatchFileSystemOkCallback: Fn(Event) -> () {}

impl<T>
  WatchFileSystemOkCallback
  for T
  where T : Fn(Event) -> () {}

enum WatchConsumerMessage {
  EventResult(result::Result<Event, NotifyError>),
  Unwatch(),
}

struct WatchFileSystenAdapterStopwatch {
  channel_sender: Sender<WatchConsumerMessage>
}

#[async_trait]
impl WatchFileSystenPortStopwatch for WatchFileSystenAdapterStopwatch {
  async fn unwatch(&mut self) -> Result<()> {
    self.channel_sender.send(WatchConsumerMessage::Unwatch()).await.unwrap();

    Ok(())
  }
}

pub struct WatchFileSystemNotifyAdapter<'a, TOkFn, TErrFn>
  where
    TOkFn: WatchFileSystemOkCallback + ?Sized,
    TErrFn: WatchFileSystemErrCallback + ?Sized,
{
  err_callback: &'a TErrFn,
  is_active: bool,
  is_recursive: bool,
  ok_callback: &'a TOkFn,
  path: path::PathBuf,
  watcher: Option<INotifyWatcher>
}

impl<'a, TOkFn, TErrFn> WatchFileSystemNotifyAdapter<'a, TOkFn, TErrFn>
  where
    TOkFn: WatchFileSystemOkCallback + ?Sized,
    TErrFn: WatchFileSystemErrCallback + ?Sized,
{
  pub fn new(
    err_callback: &'static TErrFn,
    is_recursive: bool,
    ok_callback: &'static TOkFn,
    path: path::PathBuf,
  ) -> Self {
    Self {
      err_callback,
      is_active: false,
      is_recursive,
      ok_callback,
      path,
      watcher: None,
    }
  }

  fn build_async_watcher() -> notify::Result<(RecommendedWatcher, Receiver<notify::Result<Event>>)> {
    let (mut event_channel_producer, event_channel_consumer) = channel(1);

    let watcher = RecommendedWatcher::new(move |res| {
      futures::executor::block_on(async {
        event_channel_producer.send(res).await.unwrap();
      })
    }, Config::default())?;
  
    Ok((watcher, event_channel_consumer))
  }
}

#[async_trait]
impl<'a, TOkFn, TErrFn> WatchFileSystemPort<> for WatchFileSystemNotifyAdapter<'a, TOkFn, TErrFn>
  where
  TOkFn: WatchFileSystemOkCallback + ?Sized + std::marker::Sync,
  TErrFn: WatchFileSystemErrCallback + ?Sized + std::marker::Sync,
{
  async fn watch(&mut self) -> Result<Box<dyn WatchFileSystenPortStopwatch>> {
    self.is_active = true;

    let (mut watcher, mut event_channel_consumer) = Self::build_async_watcher()?;

    let recursive_mode: RecursiveMode =
      if self.is_recursive {
        RecursiveMode::Recursive
      } else {
        RecursiveMode::NonRecursive
      };

    watcher.watch(self.path.as_ref(), recursive_mode)?;

    self.watcher = Some(watcher);

    while let Some(res) = event_channel_consumer.next().await {
      if self.is_active {
        match res {
          Ok(event) => (self.ok_callback)(event),
          Err(error) => (self.err_callback)(error),
        }
      } else {
        break;
      }
    }

    Ok(())
  }
}
