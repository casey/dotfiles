use crate::common::*;

#[derive(Clone, Deserialize)]
pub(crate) struct WindowInfo {
  pub(crate) app: String,
  pub(crate) id: u64,
  pub(crate) floating: usize,
  pub(crate) frame: Frame,
}

impl WindowInfo {
  pub(crate) fn handle(&self) -> WindowHandle {
    WindowHandle { id: self.id }
  }
}
