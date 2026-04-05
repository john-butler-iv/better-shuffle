use std::fmt::Display;

use serde::de::Error;

use super::{AlbumID, ArtistID, ExternalURLs, PlaylistID, ShowID, SpotifyURI};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct Context {
    #[serde(rename = "uri")]
    id: ContextID,
    external_urls: ExternalURLs,
    full_details_url: String,
}

impl PartialEq for Context {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for Context {}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum ContextID {
    Playlist(PlaylistID),
    Album(AlbumID),
    Artist(ArtistID),
    Show(ShowID),
}

impl From<ContextID> for SpotifyURI {
    fn from(id: ContextID) -> Self {
        match id {
            ContextID::Playlist(playlist_id) => SpotifyURI::Playlist(playlist_id),
            ContextID::Album(album_id) => SpotifyURI::Album(album_id),
            ContextID::Artist(artist_id) => SpotifyURI::Artist(artist_id),
            ContextID::Show(show_id) => SpotifyURI::Show(show_id),
        }
    }
}

impl Display for ContextID {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self {
            ContextID::Playlist(id) => write!(f, "{id}"),
            ContextID::Album(id) => write!(f, "{id}"),
            ContextID::Artist(id) => write!(f, "{id}"),
            ContextID::Show(id) => write!(f, "{id}"),
        }
    }
}

impl TryFrom<SpotifyURI> for ContextID {
    type Error = ();

    fn try_from(value: SpotifyURI) -> Result<Self, Self::Error> {
        match value {
            SpotifyURI::Playlist(id) => Ok(ContextID::Playlist(id)),
            SpotifyURI::Album(id) => Ok(ContextID::Album(id)),
            SpotifyURI::Artist(id) => Ok(ContextID::Artist(id)),
            SpotifyURI::Show(id) => Ok(ContextID::Show(id)),
            _ => Err(()),
        }
    }
}

impl TryFrom<&str> for ContextID {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        SpotifyURI::try_from(value)?.try_into()
    }
}

impl<'de> serde::Deserialize<'de> for ContextID {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        SpotifyURI::deserialize(deserializer)?
            .try_into()
            .map_err(|_| D::Error::custom("URI type is not a context"))
    }
}
