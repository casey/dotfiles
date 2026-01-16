mod app;
mod command_ext;
mod common;
mod error;
mod frame;
mod opt;
mod window_handle;
mod window_info;
mod yabai;

use crate::common::*;

// better error displays
// catch all subcommand stdout and stderr
// error on display
// include location information w/errors
// handle vs query

fn main() -> Result<(), u32> {
  match Opt::from_args().run() {
    Ok(()) => Ok(()),
    Err(err) => {
      println!("error: {}", err);
      Err(1)
    }
  }
}
