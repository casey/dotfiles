use super::*;

pub(crate) struct Worker {
  pub(crate) bar: ProgressBar,
  pub(crate) compress: bool,
  pub(crate) cursor: Arc<AtomicUsize>,
  pub(crate) destination: Utf8PathBuf,
  pub(crate) failed: Arc<AtomicBool>,
  pub(crate) files: Arc<Vec<File>>,
  pub(crate) overall: ProgressBar,
  pub(crate) source: String,
}

impl Worker {
  pub(crate) fn run(self) -> Result {
    let result = self.download_all();
    if result.is_err() {
      self.failed.store(true, Ordering::SeqCst);
    }
    self.bar.finish_and_clear();
    result
  }

  fn download_all(&self) -> Result {
    while !self.failed.load(Ordering::SeqCst) {
      let index = self.cursor.fetch_add(1, Ordering::SeqCst);

      let Some(file) = self.files.get(index) else {
        break;
      };

      self.download(file)?;
    }

    Ok(())
  }

  fn download(&self, file: &File) -> Result {
    self.bar.set_length(file.size);
    self.bar.set_position(0);
    self.bar.set_message(file.path.clone());

    let mut command = Command::new("rsync");

    command
      .arg("--partial")
      .arg("--info=progress2")
      .arg("--times")
      .arg("--files-from=-");

    if self.compress {
      command.arg("--compress");
    }

    let mut child = command
      .arg(&self.source)
      .arg(&self.destination)
      .stdin(Stdio::piped())
      .stdout(Stdio::piped())
      .stderr(Stdio::null())
      .spawn()
      .context(error::Spawn {
        path: file.path.clone(),
      })?;

    child
      .stdin
      .take()
      .unwrap()
      .write_all(format!("{}\n", file.path).as_bytes())
      .context(error::FilesFrom {
        path: file.path.clone(),
      })?;

    let mut reader = BufReader::new(child.stdout.take().unwrap());
    let mut line = Vec::new();
    let mut byte = [0u8; 1];
    let mut last = 0u64;

    loop {
      let read = reader.read(&mut byte).context(error::ProgressRead {
        path: file.path.clone(),
      })?;

      if read == 0 {
        break;
      }

      if byte[0] == b'\r' || byte[0] == b'\n' {
        if let Some(transferred) = progress::parse(&String::from_utf8_lossy(&line)) {
          let transferred = transferred.min(file.size);
          self.bar.set_position(transferred);
          self.overall.inc(transferred.saturating_sub(last));
          last = transferred;
        }
        line.clear();
      } else {
        line.push(byte[0]);
      }
    }

    let status = child.wait().context(error::Wait {
      path: file.path.clone(),
    })?;

    ensure!(
      status.success(),
      error::Download {
        path: file.path.clone(),
        status,
      }
    );

    self.bar.set_position(file.size);
    self.overall.inc(file.size.saturating_sub(last));

    Ok(())
  }
}
