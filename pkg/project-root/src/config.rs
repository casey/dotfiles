use crate::common::*;

#[derive(Deserialize)]
pub(crate) struct Config {
  pub(crate) specs: Vec<Vec<Spec>>,
}
