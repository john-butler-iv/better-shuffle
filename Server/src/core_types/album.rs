use chrono::prelude::*;
use serde::de::Error;

use super::{AlbumID, ExternalURLs, Restrictions, artist::SimplifiedArtist, image::Image};

#[derive(Debug, Clone)]
#[allow(dead_code)]
pub struct Album {
    id: AlbumID,
    name: String,
    album_type: AlbumType,
    total_tracks: u32,
    cover_arts: Vec<Image>,
    release_date: ReleaseDate,
    artists: Vec<SimplifiedArtist>,
    external_urls: ExternalURLs,
    full_details_url: String,
    restrictions: Restrictions,
}

impl PartialEq for Album {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Album {}

impl<'de> serde::Deserialize<'de> for Album {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let album_result: Result<Album, ()> = RawAlbum::deserialize(deserializer)?.try_into();
        album_result.map_err(|_| D::Error::custom("invalid inner album data"))
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
pub struct RawAlbum {
    album_type: AlbumType,
    total_tracks: u32,
    external_urls: ExternalURLs,
    href: String,
    id: AlbumID,
    images: Vec<Image>,
    name: String,
    release_date: String,
    release_date_precision: String,
    restrictions: Option<Restrictions>,
    #[serde(rename = "type")]
    _object_type: String,
    #[serde(rename = "uri")]
    _uri: String,
    artists: Vec<SimplifiedArtist>,
}

impl TryInto<Album> for RawAlbum {
    type Error = ();

    fn try_into(self) -> Result<Album, Self::Error> {
        Ok(Album {
            id: self.id,
            name: self.name,
            album_type: self.album_type,
            total_tracks: self.total_tracks,
            cover_arts: self.images,
            release_date: ReleaseDate::try_from_components(
                &self.release_date,
                &self.release_date_precision,
            )
            .ok_or(())?,
            artists: self.artists,
            external_urls: self.external_urls,
            full_details_url: self.href,
            restrictions: self.restrictions.unwrap_or(Restrictions::None),
        })
    }
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum AlbumType {
    #[serde(rename = "album")]
    Album,
    #[serde(rename = "single")]
    Single,
    #[serde(rename = "compilation")]
    Compilation,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ReleaseDate {
    Year(NaiveDate),
    Month(NaiveDate),
    Day(NaiveDate),
}

impl From<ReleaseDate> for NaiveDate {
    fn from(release_date: ReleaseDate) -> Self {
        match release_date {
            ReleaseDate::Year(date) => date,
            ReleaseDate::Month(date) => date,
            ReleaseDate::Day(date) => date,
        }
    }
}
impl AsRef<NaiveDate> for &ReleaseDate {
    fn as_ref(&self) -> &NaiveDate {
        match self {
            ReleaseDate::Year(date) => date,
            ReleaseDate::Month(date) => date,
            ReleaseDate::Day(date) => date,
        }
    }
}

impl ReleaseDate {
    pub fn try_from_components(date: &str, precision: &str) -> Option<Self> {
        let pieces: Vec<&str> = date.split('-').collect();

        match precision {
            "year" => {
                if pieces.is_empty() {
                    return None;
                }
                let year = pieces[0].parse::<i32>().ok()?;

                let date = NaiveDate::from_ymd_opt(year, 1, 1)?;

                Some(ReleaseDate::Year(date))
            }
            "month" => {
                if pieces.len() < 2 {
                    return None;
                }
                let year = pieces[0].parse::<i32>().ok()?;
                let month = pieces[1].parse::<u32>().ok()?;

                let date = NaiveDate::from_ymd_opt(year, month, 1)?;

                Some(ReleaseDate::Month(date))
            }
            "day" => {
                if pieces.len() < 3 {
                    return None;
                }
                let year = pieces[0].parse::<i32>().ok()?;
                let month = pieces[1].parse::<u32>().ok()?;
                let day = pieces[2].parse::<u32>().ok()?;

                let date = NaiveDate::from_ymd_opt(year, month, day)?;

                Some(ReleaseDate::Day(date))
            }
            _ => None,
        }
    }
}
