use reqwest::{
  blocking::{get, Client},
  StatusCode,
};
use serde::Deserialize;
use std::{fs, process::Command};

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
    for hit in result.hits {
      eprintln!("Fetching {}…", hit.image.url);

      let response = get(&hit.image.url).unwrap();

      if response.status() != StatusCode::OK {
        panic!("Bad status: {}", response.status());
      }

      let bytes = response.bytes().unwrap();

      let extension = match hit.image.url.split('.').last().unwrap() {
        "jpg" | "jpeg" => "jpg",
        "png" => "png",
        unrecognized => panic!("unrecognized: {}", unrecognized),
      };

      let jpg = format!("images/{}.{}", hit.slug, extension);
      fs::write(&jpg, bytes).unwrap();

      let description = format!("{}, {}: {}", hit.title, hit.location, hit.description);

      let status = Command::new("exiftool")
        .arg(format!("-exif:imagedescription={}", description))
        .arg("-overwrite_original")
        .arg(jpg)
        .status()
        .unwrap();

      if !status.success() {
        panic!("Bad status: {}", status);
      }
    }
  }
}
