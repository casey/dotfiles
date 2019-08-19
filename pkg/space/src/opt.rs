use crate::common::*;

#[derive(StructOpt)]
pub(crate) enum Opt {
  #[structopt(name = "setup")]
  Setup,
}

impl Opt {
  pub(crate) fn run(self) -> Result<(), Error> {
    match self {
      Self::Setup => Self::setup(),
    }
  }

  fn setup() -> Result<(), Error> {
    let alacritty = app::ALACRITTY.get_or_launch()?;

    let chrome = app::CHROME.get_or_launch()?;

    for window in yabai::windows()? {
      if window.id == alacritty.id || window.id == chrome.id {
        continue;
      }
      window.handle().set_floating(1)?;
    }

    alacritty.set_floating(0)?;

    chrome.set_floating(0)?;

    if alacritty.query()?.frame.x < chrome.query()?.frame.x {
      alacritty.swap(&chrome)?;
    }

    Ok(())
  }
}
