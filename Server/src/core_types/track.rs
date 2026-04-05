use std::time::Duration;

use crate::core_types::SpotifyURI;

use super::{
    ExternalIDs, ExternalURLs, Restrictions, TrackID, album::Album, artist::SimplifiedArtist,
};

#[derive(Debug, Clone, Eq)]
#[allow(dead_code)]
pub struct Track {
    id: TrackID,
    name: String,
    duration: Duration,
    album: Album,
    artists: Vec<SimplifiedArtist>,
    disc_number: u32,
    track_number: u32,
    is_explicit: bool,
    is_playable: bool,
    is_local: bool,
    external_ids: ExternalIDs,
    external_urls: ExternalURLs,
    full_details_url: String,
    restrictions: Restrictions,
}

impl<'de> serde::Deserialize<'de> for Track {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(RawTrack::deserialize(deserializer)?.into())
    }
}
#[derive(Debug, Clone, serde::Deserialize)]
struct RawTrack {
    id: TrackID,
    name: String,
    duration_ms: u64,
    album: Album,
    artists: Vec<SimplifiedArtist>,
    disc_number: u32,
    track_number: u32,
    is_explicit: Option<bool>,
    is_playable: Option<bool>,
    is_local: Option<bool>,
    external_ids: ExternalIDs,
    external_urls: ExternalURLs,
    href: String,
    restrictions: Option<Restrictions>,
}

impl From<RawTrack> for Track {
    fn from(raw_track: RawTrack) -> Self {
        Track {
            id: raw_track.id,
            name: raw_track.name,
            duration: Duration::from_millis(raw_track.duration_ms),
            album: raw_track.album,
            artists: raw_track.artists,
            disc_number: raw_track.disc_number,
            track_number: raw_track.track_number,
            is_explicit: raw_track.is_explicit.unwrap_or(false),
            is_playable: raw_track.is_playable.unwrap_or(false),
            is_local: raw_track.is_local.unwrap_or(false),
            external_ids: raw_track.external_ids,
            external_urls: raw_track.external_urls,
            full_details_url: raw_track.href,
            restrictions: raw_track.restrictions.unwrap_or(Restrictions::None),
        }
    }
}

impl Track {
    #[allow(dead_code)]
    pub fn uri(&self) -> SpotifyURI {
        SpotifyURI::Track(self.id.clone())
    }

    #[allow(dead_code)]
    pub fn id(&self) -> &TrackID {
        &self.id
    }
    pub fn duration(&self) -> &Duration {
        &self.duration
    }
}

impl PartialEq<TrackID> for Track {
    fn eq(&self, other: &TrackID) -> bool {
        &self.id == other
    }
}

impl PartialEq for Track {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
