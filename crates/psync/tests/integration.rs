use {
  std::{fs, process::Command},
  tempfile::TempDir,
};

#[test]
fn copies_files() {
  let source = TempDir::new().unwrap();
  let destination = TempDir::new().unwrap();

  let contents = vec![b'a'; 1 << 20];
  fs::write(source.path().join("foo"), &contents).unwrap();

  let status = Command::new(env!("CARGO_BIN_EXE_psync"))
    .arg(source.path())
    .arg(destination.path())
    .status()
    .unwrap();

  assert!(status.success());

  assert_eq!(fs::read(destination.path().join("foo")).unwrap(), contents);
}
