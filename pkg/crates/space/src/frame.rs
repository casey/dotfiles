use crate::common::*;

#[derive(Deserialize, Clone)]
pub(crate) struct Frame {
  pub(crate) x: f64,
  pub(crate) y: f64,
  pub(crate) w: f64,
  pub(crate) h: f64,
}
