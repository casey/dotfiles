use std::{
  env, fs,
  path::{Path, PathBuf},
  process::Command,
};

const VERSION: &str = "6.1.0";
const SHA256: &str = "b66b25aeb4df84e33199dc21694014d336d222cbd9deb0e5a7c14bd6aa0d0fd0";

fn main() {
  println!("cargo::rerun-if-changed=build.rs");

  let out = PathBuf::from(env::var("OUT_DIR").unwrap()).join("echarts.min.js");

  if out.exists() {
    return;
  }

  let cache = dirs::cache_dir()
    .expect("failed to locate user cache directory")
    .join("stars")
    .join(format!("echarts-{VERSION}.min.js"));

  if !cache.exists() || sha256(&cache) != SHA256 {
    download(&cache);
  }

  assert_eq!(sha256(&cache), SHA256, "echarts hash mismatch");

  fs::copy(&cache, &out).expect("failed to copy echarts to OUT_DIR");
}

fn download(cache: &Path) {
  fs::create_dir_all(cache.parent().unwrap()).expect("failed to create cache directory");

  let status = Command::new("curl")
    .args([
      "-fsSL",
      &format!("https://cdn.jsdelivr.net/npm/echarts@{VERSION}/dist/echarts.min.js"),
      "-o",
    ])
    .arg(cache)
    .status()
    .expect("failed to run curl");

  assert!(status.success(), "curl failed: {status}");
}

fn sha256(path: &Path) -> String {
  let output = Command::new("shasum")
    .args(["-a", "256"])
    .arg(path)
    .output()
    .expect("failed to run shasum");

  assert!(output.status.success(), "shasum failed: {}", output.status);

  String::from_utf8(output.stdout)
    .unwrap()
    .split_whitespace()
    .next()
    .expect("empty shasum output")
    .to_string()
}
