use crate::common::*;

pub(crate) struct Track {
  pub(crate) mp3:          PathBuf,
  pub(crate) flac:         Option<PathBuf>,
  pub(crate) added:        DateTime<Utc>,
  pub(crate) disc:         u32,
  pub(crate) track:        u32,
  pub(crate) album:        String,
  pub(crate) album_artist: String,
  pub(crate) year:         i32,
}

impl Track {
  pub(crate) fn new(mp3: PathBuf, flac: Option<PathBuf>) -> Result<Track> {
    assert_eq!(mp3.extension(), Some(OsStr::new("mp3")));

    if let Some(flac) = &flac {
      assert_eq!(flac.extension(), Some(OsStr::new("flac")));
    }

    let tag = Tag::read_from_path(&mp3).context(error::Id3 { path: &mp3 })?;

    let comments = tag
      .comments()
      .filter(|comment| comment.description.is_empty())
      .map(|comment| comment.text.clone())
      .collect::<Vec<String>>();

    if comments.len() != 1 {
      return Err(Error::Comments {
        path: mp3,
        comments,
      });
    }

    let timestamp = comments[0]
      .trim_matches('\u{0}')
      .parse::<u64>()
      .context(error::Timestamp {
        path: &mp3,
        text: &comments[0],
      })?;

    let naive = NaiveDateTime::from_timestamp(timestamp as i64, 0);

    let added: DateTime<Utc> = DateTime::from_utc(naive, Utc);

    let disc = {
      let number = tag.disc();

      match number {
        Some(0) | None => return Err(Error::DiscNumber { path: mp3, number }),
        Some(n) => n - 1,
      }
    };

    let track = {
      let number = tag.track();

      match number {
        Some(0) | None => return Err(Error::TrackNumber { path: mp3, number }),
        Some(n) => n - 1,
      }
    };

    let album = tag
      .album()
      .ok_or_else(|| Error::TrackAlbum {
        path: mp3.to_owned(),
      })?
      .to_owned();

    if album.is_empty() {
      return Err(Error::TrackAlbum {
        path: mp3.to_owned(),
      });
    }

    let album_artist = tag
      .album_artist()
      .ok_or_else(|| Error::TrackAlbumArtist {
        path: mp3.to_owned(),
      })?
      .to_owned();

    if album_artist.is_empty() {
      return Err(Error::TrackAlbumArtist {
        path: mp3.to_owned(),
      });
    }

    let year = tag
      .date_recorded()
      .map(|timestamp| timestamp.year)
      .or_else(|| tag.year())
      .ok_or_else(|| Error::TrackYear { path: mp3.clone() })?;

    Ok(Track {
      mp3,
      flac,
      added,
      disc,
      track,
      album,
      year,
      album_artist,
    })
  }

  pub(crate) fn album_key(&self) -> (i32, &str, &str) {
    (self.year, &self.album, &self.album_artist)
  }

  pub(crate) fn track_key(&self) -> (u32, u32) {
    (self.disc, self.track)
  }
}
