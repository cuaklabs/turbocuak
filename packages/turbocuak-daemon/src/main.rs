#[macro_use]
extern crate log;

use clap::Parser;
use filesystem::domain::port::WatchFileSystemPort;
use filesystem::infrastructure::notify::adapter::{WatchFileSystemNotifyAdapter, WatchFileSystemOkCallback, WatchFileSystemErrCallback};
use futures::join;
use monorepo_state::application::command_handler::monorepo_state_process_command_handler;
use notify::Error as NotifyError;

use common::domain::model::Result;
use monorepo_state::domain::model::MonorepoState;
use notify::Event;

mod common;
mod config;
mod filesystem;
mod monorepo_state;

#[derive(Parser)]
struct CliArgs {
  #[clap(long = "root-directory", parse(from_os_str))]
  root_directory: std::path::PathBuf,
}

#[actix_web::main]
async fn main() -> Result<()> {
  let cli: CliArgs = CliArgs::parse();

  let monorepo_state: MonorepoState =
    monorepo_state_process_command_handler(&cli.root_directory)
      .unwrap()
      .monorepo_state
      .lock()
      .unwrap()
      .take()
      .unwrap()
  ;

  let mut watch_file_system_port: WatchFileSystemNotifyAdapter<
    dyn WatchFileSystemOkCallback + std::marker::Sync,
    dyn WatchFileSystemErrCallback + std::marker::Sync
  > =
    WatchFileSystemNotifyAdapter::new(
      &err_callback,
      false,
      &ok_callback,
      monorepo_state.root_directory,
    );

  let (
    mut watch,
    mut stopwatch,
  ) = (&mut watch_file_system_port as & mut dyn WatchFileSystemPort).prepare()?;

  let (
    watch_result,
    //unwatch_result,
  ) = join!(
    watch.watch(),
    //stopwatch.unwatch(),
  );

  watch_result?;
  //unwatch_result?;

  Ok(())
}

fn ok_callback(event: Event) {
  println!("changed: {:?}", event);
}

fn err_callback(error: NotifyError) {
  println!("watch error: {:?}", error);
}
