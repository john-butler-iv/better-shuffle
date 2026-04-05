use super::{ArtistID, ExternalURLs};

#[derive(Debug, Clone, serde::Deserialize)]
pub struct SimplifiedArtist {
    id: ArtistID,
    name: String,

    external_urls: ExternalURLs,
    #[serde(rename = "href")]
    full_details_url: String,
}

impl PartialEq for SimplifiedArtist {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}
impl Eq for SimplifiedArtist {}
