use crate::common::*;

#[derive(Deserialize, Clone)]
pub(crate) struct Frame {
  pub(crate) x: f64,
  #[allow(dead_code)]
  pub(crate) y: f64,
  #[allow(dead_code)]
  pub(crate) w: f64,
  #[allow(dead_code)]
  pub(crate) h: f64,
}
