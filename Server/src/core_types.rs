pub mod album;
pub mod artist;
pub mod context;
pub mod device;
pub mod episode;
pub mod image;
pub mod playback_state;
pub mod track;

use std::fmt::Display;

use serde::{Serialize, de::Error};

use crate::core_types::device::Device;

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum SpotifyURI {
    Playlist(PlaylistID),
    Album(AlbumID),
    Track(TrackID),
    Episode(EpisodeID),
    Show(ShowID),
    Artist(ArtistID),
}

impl TryFrom<&str> for SpotifyURI {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SpotifyURI::try_from(value.to_string())
    }
}

impl TryFrom<String> for SpotifyURI {
    type Error = ();

    fn try_from(value: String) -> Result<Self, Self::Error> {
        let parts: Vec<&str> = value.split(':').collect();
        if parts.len() != 3 {
            return Err(());
        }
        match parts[1] {
            "playlist" => Ok(SpotifyURI::Playlist(PlaylistID(SpotifyID(
                parts[2].to_string(),
            )))),
            "album" => Ok(SpotifyURI::Album(AlbumID(SpotifyID(parts[2].to_string())))),
            "track" => Ok(SpotifyURI::Track(TrackID(SpotifyID(parts[2].to_string())))),
            "artist" => Ok(SpotifyURI::Artist(ArtistID(SpotifyID(
                parts[2].to_string(),
            )))),
            "show" => Ok(SpotifyURI::Show(ShowID(SpotifyID(parts[2].to_string())))),
            "episode" => Ok(SpotifyURI::Episode(EpisodeID(SpotifyID(
                parts[2].to_string(),
            )))),
            _ => Err(()),
        }
    }
}

impl Display for SpotifyURI {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            SpotifyURI::Playlist(id) => write!(f, "spotify:playlist:{id}"),
            SpotifyURI::Album(id) => write!(f, "spotify:album:{id}"),
            SpotifyURI::Track(id) => write!(f, "spotify:track:{id}"),
            SpotifyURI::Episode(id) => write!(f, "spotify:episode:{id}"),
            SpotifyURI::Show(id) => write!(f, "spotify:show:{id}"),
            SpotifyURI::Artist(id) => write!(f, "spotify:artist:{id}"),
        }
    }
}

impl<'de> serde::Deserialize<'de> for SpotifyURI {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        let uri_str = String::deserialize(deserializer)?;
        SpotifyURI::try_from(uri_str.as_str()).map_err(|_| D::Error::custom("invalid Spotify URI"))
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct PlaylistID(pub SpotifyID);
impl Display for PlaylistID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<PlaylistID> for SpotifyURI {
    fn from(id: PlaylistID) -> Self {
        SpotifyURI::Playlist(id)
    }
}

impl PlaylistID {
    fn chill_id() -> &'static str {
        "6gZvhzSzX2XnH7Qo5lW1im"
    }
    pub fn cmon_man_chill_out() -> Self {
        PlaylistID(SpotifyID(Self::chill_id().to_string()))
    }
    pub fn recently_deleted_chill() -> Self {
        PlaylistID(SpotifyID("2PduIdMiWBFTiLGfYez59x".to_string()))
    }

    fn high_energy_id() -> &'static str {
        "1q22DAb8f8vJiPhso3awoO"
    }
    pub fn high_energy() -> Self {
        PlaylistID(SpotifyID(Self::high_energy_id().to_string()))
    }
    pub fn recently_deleted_hype() -> Self {
        PlaylistID(SpotifyID("5TYJTZc4S6OfOQ6iPwhyZs".to_string()))
    }

    pub fn discover_weekly() -> Self {
        PlaylistID(SpotifyID("37i9dQZEVXcWj5pDQXaurQ".to_string()))
    }
    pub fn discover_holding_queue() -> Self {
        PlaylistID(SpotifyID("2HYWPN8Ub0rnbuOZJvjmN5".to_string()))
    }

    pub fn get_recently_deleted(playlist: &PlaylistID) -> Option<Self> {
        if playlist == &Self::cmon_man_chill_out() {
            Some(Self::recently_deleted_chill())
        } else if playlist == &Self::high_energy() {
            Some(Self::recently_deleted_hype())
        } else {
            None
        }
    }

    pub fn copy_id(&self) -> SpotifyID {
        self.0.clone()
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct AlbumID(SpotifyID);
impl Display for AlbumID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl From<AlbumID> for SpotifyURI {
    fn from(id: AlbumID) -> Self {
        SpotifyURI::Album(id)
    }
}

#[derive(Hash, Debug, Clone, PartialEq, Eq, serde::Deserialize, serde::Serialize)]
pub struct TrackID(SpotifyID);
impl Display for TrackID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<TrackID> for SpotifyURI {
    fn from(id: TrackID) -> Self {
        SpotifyURI::Track(id)
    }
}

impl TryFrom<&str> for TrackID {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(uri) = SpotifyURI::try_from(value) {
            if let SpotifyURI::Track(id) = uri {
                Ok(id)
            } else {
                Err(())
            }
        } else {
            Ok(TrackID(SpotifyID(value.to_string())))
        }
    }
}

#[derive(Debug, Clone, PartialEq, Eq, serde::Deserialize)]
pub struct ArtistID(SpotifyID);
impl Display for ArtistID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl From<ArtistID> for SpotifyURI {
    fn from(id: ArtistID) -> Self {
        SpotifyURI::Artist(id)
    }
}

impl TryFrom<&str> for ArtistID {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(uri) = SpotifyURI::try_from(value) {
            if let SpotifyURI::Artist(id) = uri {
                Ok(id)
            } else {
                Err(())
            }
        } else {
            Ok(ArtistID(SpotifyID(value.to_string())))
        }
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq)]
pub struct ShowID(SpotifyID);
impl Display for ShowID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl TryFrom<&str> for ShowID {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(uri) = SpotifyURI::try_from(value) {
            if let SpotifyURI::Show(id) = uri {
                Ok(id)
            } else {
                Err(())
            }
        } else {
            Ok(ShowID(SpotifyID(value.to_string())))
        }
    }
}

impl From<ShowID> for SpotifyURI {
    fn from(id: ShowID) -> Self {
        SpotifyURI::Show(id)
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq)]
pub struct EpisodeID(SpotifyID);
impl Display for EpisodeID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}
impl TryFrom<&str> for EpisodeID {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        if let Ok(uri) = SpotifyURI::try_from(value) {
            if let SpotifyURI::Episode(id) = uri {
                Ok(id)
            } else {
                Err(())
            }
        } else {
            Ok(EpisodeID(SpotifyID(value.to_string())))
        }
    }
}

impl From<EpisodeID> for SpotifyURI {
    fn from(id: EpisodeID) -> Self {
        SpotifyURI::Episode(id)
    }
}

#[derive(Hash, Debug, Clone, serde::Deserialize, serde::Serialize, PartialEq, Eq)]
pub struct SpotifyID(pub String);
impl Display for SpotifyID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq)]
pub struct DeviceID(SpotifyID);
impl Display for DeviceID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "{}", self.0)
    }
}

impl Serialize for DeviceID {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        self.0.serialize(serializer)
    }
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq)]
pub enum Restrictions {
    #[serde(rename = "market")]
    Market,
    #[serde(rename = "product")]
    Product,
    #[serde(rename = "explicit")]
    Explicit,
    None,
    #[serde(other)]
    Unknown,
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq)]
pub struct ExternalURLs {
    spotify: String,
}

#[derive(Debug, Clone, serde::Deserialize, PartialEq, Eq)]
pub struct ExternalIDs {
    isrc: Option<String>,
    ean: Option<String>,
    upc: Option<String>,
}
