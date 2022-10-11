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
use crate::filesystem::domain::port::{
  WatchFileSystemPort,
  WatchFileSystenPortStopwatch,
  WatchFileSystenPortWatch
};

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

impl WatchFileSystenAdapterStopwatch {
  pub fn new(channel_sender: Sender<WatchConsumerMessage>) -> Self {
    Self { channel_sender }
  }
}

#[async_trait]
impl WatchFileSystenPortStopwatch for WatchFileSystenAdapterStopwatch {
  async fn unwatch(&mut self) -> Result<()> {
    self.channel_sender.send(WatchConsumerMessage::Unwatch()).await.unwrap();

    Ok(())
  }
}

struct WatchFileSystenAdapterWatch<'a, TOkFn, TErrFn>
  where
  TOkFn: WatchFileSystemOkCallback + ?Sized,
  TErrFn: WatchFileSystemErrCallback + ?Sized,
{
  channel_consumer: Receiver<WatchConsumerMessage>,
  err_callback: &'a TErrFn,
  ok_callback: &'a TOkFn,
  path: &'a path::PathBuf,
  recursive_mode: RecursiveMode,
  watcher: INotifyWatcher,
}

impl<'a, TOkFn, TErrFn> WatchFileSystenAdapterWatch<'a, TOkFn, TErrFn>
  where
    TOkFn: WatchFileSystemOkCallback + ?Sized,
    TErrFn: WatchFileSystemErrCallback + ?Sized,
{
  pub fn new(
    channel_consumer: Receiver<WatchConsumerMessage>,
    err_callback: &'a TErrFn,
    ok_callback: &'a TOkFn,
    path: &'a path::PathBuf,
    recursive_mode: RecursiveMode,
    watcher: INotifyWatcher,
  ) -> Self {
    Self {
      channel_consumer,
      err_callback,
      ok_callback,
      path,
      recursive_mode,
      watcher
    }
  }
}

#[async_trait]
impl<'a, TOkFn, TErrFn> WatchFileSystenPortWatch for WatchFileSystenAdapterWatch<'a, TOkFn, TErrFn>
where
  TOkFn: WatchFileSystemOkCallback + ?Sized + std::marker::Sync,
  TErrFn: WatchFileSystemErrCallback + ?Sized + std::marker::Sync,
{
  async fn watch(&mut self) -> Result<()> {
    self.watcher.watch(self.path, self.recursive_mode)?;

    while let Some(res) = self.channel_consumer.next().await {
      match res {
        WatchConsumerMessage::EventResult(Ok(event)) => (self.ok_callback)(event),
        WatchConsumerMessage::EventResult(Err(error)) => (self.err_callback)(error),
        WatchConsumerMessage::Unwatch() => { 
          self.watcher.unwatch(self.path)?;
          break;
        }
      }
    }

    Ok(())
  }
}

pub struct WatchFileSystemNotifyAdapter<'a, TOkFn, TErrFn>
  where
    TOkFn: WatchFileSystemOkCallback + ?Sized,
    TErrFn: WatchFileSystemErrCallback + ?Sized,
{
  err_callback: &'a TErrFn,
  is_recursive: bool,
  ok_callback: &'a TOkFn,
  path: path::PathBuf,
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
      is_recursive,
      ok_callback,
      path,
    }
  }

  fn build_async_watcher() -> notify::Result<
    (RecommendedWatcher, Sender<WatchConsumerMessage>, Receiver<WatchConsumerMessage>
  )> {
    let (mut channel_watcher_producer, channel_consumer) = channel(1);

    let channel_stopwatch_producer = channel_watcher_producer.clone();

    let watcher = RecommendedWatcher::new(move |res| {
      futures::executor::block_on(async {
        channel_watcher_producer.send(WatchConsumerMessage::EventResult(res)).await.unwrap();
      })
    }, Config::default())?;
  
    Ok((watcher, channel_stopwatch_producer, channel_consumer))
  }
}

impl<'a, TOkFn, TErrFn> WatchFileSystemPort for WatchFileSystemNotifyAdapter<'a, TOkFn, TErrFn>
  where
  TOkFn: WatchFileSystemOkCallback + ?Sized + std::marker::Sync,
  TErrFn: WatchFileSystemErrCallback + ?Sized + std::marker::Sync,
{
  fn prepare(&mut self) -> Result<
    (Box<dyn WatchFileSystenPortWatch + '_>, Box<dyn WatchFileSystenPortStopwatch + '_>)
  > {
    let (
      mut watcher,
      channel_stopwatch_producer,
      channel_consumer,
    ) = Self::build_async_watcher()?;

    let recursive_mode: RecursiveMode =
      if self.is_recursive {
        RecursiveMode::Recursive
      } else {
        RecursiveMode::NonRecursive
      };

    watcher.watch(self.path.as_ref(), recursive_mode)?;

    let watch_box: Box<dyn WatchFileSystenPortWatch> = Box::new(
      WatchFileSystenAdapterWatch::new(
        channel_consumer,
        &self.err_callback,
        &self.ok_callback,
        &self.path,
        recursive_mode,
        watcher,
      ),
    );
    let unwatch_box: Box<dyn WatchFileSystenPortStopwatch> = Box::new(
      WatchFileSystenAdapterStopwatch::new(channel_stopwatch_producer),
    );

    let result_tuple = (
      watch_box,
      unwatch_box,
    );

    Ok(result_tuple)
  }
}
