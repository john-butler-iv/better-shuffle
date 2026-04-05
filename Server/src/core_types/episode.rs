use crate::core_types::SpotifyURI;

use super::EpisodeID;

#[derive(Debug, Clone, serde::Deserialize, Eq)]
pub struct Episode {
    id: EpisodeID,
}
impl Episode {
    pub fn uri(&self) -> SpotifyURI {
        SpotifyURI::Episode(self.id.clone())
    }
}

impl PartialEq<EpisodeID> for Episode {
    fn eq(&self, other: &EpisodeID) -> bool {
        &self.id == other
    }
}
impl PartialEq for Episode {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
