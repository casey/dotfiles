use reqwest::{
  blocking::{get, Client},
  StatusCode,
};
use serde::Deserialize;
use std::{fs, path::Path, process::Command};

const URL: &str = "https://ai7o5ij8d5-dsn.algolia.net/1/indexes/*/queries?&x-algolia-application-id=AI7O5IJ8D5&x-algolia-api-key=7f1a509e834f885835edcfd3482b990c";
const BODY: &str = r#"{"requests":[{"indexName":"overview","params":"hitsPerPage=1000"}]}"#;

#[derive(Deserialize)]
struct Response {
  results: Vec<Contents>,
}

#[derive(Deserialize)]
struct Contents {
  hits: Vec<Hit>,
}

#[derive(Deserialize)]
struct Hit {
  slug:        String,
  title:       String,
  location:    String,
  description: String,
  image:       Image,
}

#[derive(Deserialize)]
struct Image {
  url: String,
}

fn main() {
  let client = Client::new();

  let response = client.post(URL).body(BODY).send().unwrap();

  if response.status() != StatusCode::OK {
    panic!("Bad status: {}", response.status());
  }

  let text = response.text().unwrap();

  let response = serde_json::from_str::<Response>(&text).unwrap();

  for result in response.results {
    for (i, hit) in result.hits.iter().enumerate() {
      eprintln!(
        "Fetching {}/{}: {}…",
        i + 1,
        result.hits.len(),
        hit.image.url
      );

      let extension = match hit.image.url.split('.').last().unwrap() {
        "jpg" | "jpeg" => "jpg",
        "png" => "png",
        "gif" => "gif",
        unrecognized => panic!("unrecognized: {}", unrecognized),
      };

      let dst = format!("images/{}.{}", hit.slug, extension);

      if Path::new(&dst).is_file() {
        eprintln!("Already fetched!");
        continue;
      }

      let response = get(&hit.image.url).unwrap();

      if response.status() != StatusCode::OK {
        panic!("Bad status: {}", response.status());
      }

      let bytes = response.bytes().unwrap();

      let dst = format!("images/{}.{}", hit.slug, extension);
      fs::write(&dst, bytes).unwrap();

      let description = format!("{}, {}: {}", hit.title, hit.location, hit.description);

      let status = Command::new("exiftool")
        .arg(format!("-exif:imagedescription={}", description))
        .arg("-overwrite_original")
        .arg(dst)
        .status()
        .unwrap();

      if !status.success() {
        panic!("Bad status: {}", status);
      }
    }
  }
}
