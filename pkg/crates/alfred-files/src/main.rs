use std::path::PathBuf;

use serde::Serialize;
use walkdir::WalkDir;

#[derive(Serialize)]
struct Output {
  items: Vec<Item>,
}

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq)]
struct Item {
  uid:          PathBuf,
  #[serde(rename = "type")]
  ty:           ItemType,
  title:        String,
  subtitle:     PathBuf,
  arg:          PathBuf,
  autocomplete: String,
  icon:         Icon,
}

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq)]
struct Icon {
  #[serde(rename = "type")]
  ty:   IconType,
  path: PathBuf,
}

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
enum IconType {
  FileIcon,
}

#[derive(Serialize, Ord, PartialOrd, Eq, PartialEq)]
#[serde(rename_all = "lowercase")]
enum ItemType {
  File,
}

const ROOTS: &[(&str, usize)] = &[
  ("Desktop", 0),
  ("Dropbox", 1),
  ("Library", 0),
  ("dat", 1),
  ("src", 1),
  ("src/txt", 1),
];

fn main() {
  let mut items = Vec::new();

  for &(root, depth) in ROOTS {
    let path = dirs::home_dir().unwrap().join(root);

    for result in WalkDir::new(&path).max_depth(depth) {
      let entry = result.unwrap();
      let path = entry.path().to_owned();
      let title = entry.file_name().to_string_lossy().into_owned();

      items.push(Item {
        uid: path.clone(),
        ty: ItemType::File,
        subtitle: path.clone(),
        arg: path.clone(),
        autocomplete: title.clone(),
        icon: Icon {
          ty:   IconType::FileIcon,
          path: path.clone(),
        },
        title,
      });
    }
  }

  items.sort();
  items.dedup();

  let output = Output { items };

  serde_json::to_writer_pretty(std::io::stdout(), &output).unwrap();
  println!();
}
