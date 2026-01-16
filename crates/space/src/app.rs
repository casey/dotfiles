use crate::common::*;

#[derive(Copy, Clone)]
pub(crate) struct App {
  pub(crate) name: &'static str,
  pub(crate) binary: &'static str,
}

pub(crate) const ALACRITTY: App = App {
  name: "Alacritty",
  binary: "/Applications/Alacritty.app/Contents/MacOS/alacritty",
};

pub(crate) const CHROME: App = App {
  name: "Google Chrome",
  binary: "/Applications/Google Chrome.app/Contents/MacOS/Google Chrome",
};

impl App {
  pub(crate) fn get_or_launch(self) -> Result<WindowHandle, Error> {
    let start = Instant::now();

    while start.elapsed() < Duration::from_millis(500) {
      for window in yabai::windows()? {
        if window.app == self.name {
          return Ok(window.handle());
        }
      }

      let exit_status = Command::new(self.binary).status()?;

      if !exit_status.success() {
        return Err(Error::ExitStatus { exit_status });
      }
    }

    for window in yabai::windows()? {
      if window.app == self.name {
        return Ok(window.handle());
      }
    }

    let start = Instant::now();

    loop {
      if start.elapsed() > Duration::from_millis(500) {
        return Err(Error::Launch {
          binary: self.binary.to_owned(),
        });
      }
    }
  }
}
