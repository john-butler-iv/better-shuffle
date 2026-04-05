#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
pub struct Image {
    url: String,
    height: Option<u32>,
    width: Option<u32>,
}
