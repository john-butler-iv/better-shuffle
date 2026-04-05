#[derive(serde::Deserialize)]
struct FileEnvironment {
    client_id: String,
    client_secret: String,
    known_refresh_token: String,
}
pub struct Environment {
    pub client_id: String,
    pub client_secret: String,
    pub redirect_port: u16,
    pub oauth_redirect_path: String,
    pub known_refresh_token: String,
    pub playlist_data_path: String,
}

impl Environment {
    pub fn oauth_redirect_uri(&self) -> String {
        format!(
            "http://127.0.0.1:{}/{}",
            self.redirect_port, self.oauth_redirect_path
        )
    }
}

const OAUTH_REDIRECT_PATH: &str = "auth_callback";

pub fn env_vars() -> Environment {
    let file_data: FileEnvironment =
        serde_json::from_str(include_str!("../env.json")).expect("failed to parse env.json");

    Environment {
        client_id: file_data.client_id,
        client_secret: file_data.client_secret,
        redirect_port: 3000,
        oauth_redirect_path: OAUTH_REDIRECT_PATH.to_string(),
        known_refresh_token: file_data.known_refresh_token,
        playlist_data_path: "./playlist_data".to_string(),
    }
}
