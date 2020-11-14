use crate::common::*;

#[throws]
pub(crate) fn rename(src: &Path, dst: &Path) {
  if dst.exists() {
    bail!("Rename destination exists: `{}`", dst.display());
  }

  std::fs::rename(src, dst).with_context(|| {
    anyhow!(
      "Rename failed: src `{}`, dst: `{}",
      src.display(),
      dst.display()
    )
  })?;
}
