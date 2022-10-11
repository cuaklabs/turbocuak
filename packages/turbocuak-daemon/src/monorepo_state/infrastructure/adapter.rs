use std::path::{PathBuf, self};
use std::sync;

use notify::event;

use crate::common::domain::action::InteractionFn;
use crate::common::domain::model::Result;
use crate::filesystem::domain::port::{WatchFileSystemPort, WatchFileSystenPortWatch, WatchFileSystenPortStopwatch};
use crate::filesystem::infrastructure::notify::adapter::{
  WatchFileSystemNotifyAdapter,
  WatchFileSystemOkCallback,
  WatchFileSystemErrCallback,
};
use crate::monorepo_state::domain::interaction::parse_monorepo_state_interaction;
use crate::monorepo_state::domain::model::MonorepoState;

pub struct MonitorMonorepoStateAdapter {
  monorepo_state_shared: sync::Arc<sync::Mutex<Option<MonorepoState>>>,
  watch_file_system_port: Box<dyn WatchFileSystemPort>,
}

impl MonitorMonorepoStateAdapter {
  pub fn new(
    monorepo_state_shared: sync::Arc<sync::Mutex<Option<MonorepoState>>>,
    path: PathBuf,
  ) -> Self {
    let watch_file_system_adapter: WatchFileSystemNotifyAdapter<
      dyn WatchFileSystemOkCallback + std::marker::Sync,
      dyn WatchFileSystemErrCallback + std::marker::Sync
    > = WatchFileSystemNotifyAdapter::new(
      &Self::err_callback,
      false,
      &Self::ok_callback_generator(
        &parse_monorepo_state_interaction,
        monorepo_state_shared,
        path::PathBuf::from(&path),
      ),
      path,
    );

    Self {
      monorepo_state_shared,
      watch_file_system_port: Box::new(watch_file_system_adapter),
    }
  }

  fn err_callback(error: notify::Error) {

  }

  fn should_invalidate_cache(event: &event::Event) -> bool {
    match event.kind {
      event::EventKind::Create(_) => true,
      event::EventKind::Modify(_) => true,
      event::EventKind::Remove(_) => true,
      _ => false
    }
  }

  fn is_event_targeting_path(event: &event::Event, path: &path::Path) -> bool {
    event.paths.iter().map(
      |event_path: &PathBuf| -> bool {
        event_path.canonicalize().unwrap() == path
      }
    ).any(|result: bool| -> bool { result })
  }

  fn ok_callback_generator<'a: 'b, 'b>(
    parse_monorepo_state_interaction: &'static (dyn InteractionFn<&'b path::Path, MonorepoState> + Sync),
    monorepo_state_shared: sync::Arc<sync::Mutex<Option<MonorepoState>>>,
    path: path::PathBuf,
  ) -> impl WatchFileSystemOkCallback + Sync + 'b {
    move |event: event::Event| -> () {
      let files_updated: bool = Self::should_invalidate_cache(&event);
      let is_global_config_path: bool = Self::is_event_targeting_path(&event, &path);
  
      if files_updated && is_global_config_path {
        let monorepo_state_result: Result<MonorepoState> = parse_monorepo_state_interaction(&path);

        if let Ok(monorepo_state) = monorepo_state_result {
          monorepo_state_shared.lock().unwrap().replace(monorepo_state);
        }
      }
    }
  }
}

impl WatchFileSystemPort for MonitorMonorepoStateAdapter {
  fn prepare(&mut self) -> Result<
  (Box<dyn WatchFileSystenPortWatch + '_>, Box<dyn WatchFileSystenPortStopwatch + '_>)
> {
    self.watch_file_system_port.prepare()
  }
}
