use crate::common::*;

pub(crate) fn rename(src: &Path, dst: &Path) -> Result<()> {
  eprintln!("Renaming\n{}\n{}", src.display(), dst.display());

  if dst.exists() {
    return Err(Error::RenameDstExists {
      path: dst.to_owned(),
    });
  }

  std::fs::rename(src, dst).context(error::Rename { src, dst })
}
