use std::{fmt::Display, time::SystemTime};

use base64::{Engine, engine::general_purpose};

use crate::spotify_uri_bindings;

#[derive(Debug, Clone)]
pub enum Token {
    PreAuthed,
    Bearer(BearerToken),
}

#[derive(Debug, Clone)]
pub struct BearerToken {
    access_token: String,
    expires_at: SystemTime,
    scopes: Vec<Scope>,
    refresh_token: Option<String>,
}

impl Token {
    pub async fn auth_header(&mut self) -> Result<&str, Box<dyn std::error::Error>> {
        match self {
            Token::PreAuthed => Err("no auth available".into()),
            Token::Bearer(bearer) => bearer.auth_header().await,
        }
    }

    pub fn contains_scope(&self, scope: &Scope) -> bool {
        match self {
            Token::PreAuthed => false,
            Token::Bearer(bearer) => bearer.contains_scope(scope),
        }
    }
}

impl BearerToken {
    pub async fn from_environment_refresh_token() -> Option<BearerToken> {
        let env = crate::env::env_vars();
        Self::from_refresh_token(env.known_refresh_token.clone()).await
    }
    pub async fn from_refresh_token(refresh_token: String) -> Option<BearerToken> {
        let mut token = BearerToken {
            access_token: String::new(),
            expires_at: SystemTime::now(),
            scopes: Vec::new(),
            refresh_token: Some(refresh_token),
        };
        force_refresh_token(&mut token).await.ok()?;
        if token.access_token.is_empty() {
            None
        } else {
            Some(token)
        }
    }
    pub async fn auth_header(&mut self) -> Result<&str, Box<dyn std::error::Error>> {
        refresh_token(self).await?;
        Ok(&self.access_token)
    }

    pub fn contains_scope(&self, scope: &Scope) -> bool {
        self.scopes.contains(scope)
    }

    pub fn contains_scopes(&self, scopes: &[Scope]) -> bool {
        scopes.iter().all(|s| self.contains_scope(s))
    }
}

#[derive(serde::Deserialize, Debug)]
struct RawAuthResponse {
    access_token: String,
    expires_in: u64,
    scope: Option<String>,
    refresh_token: Option<String>,
}

impl From<RawAuthResponse> for Token {
    fn from(response: RawAuthResponse) -> Token {
        Token::Bearer(response.into())
    }
}
impl From<RawAuthResponse> for BearerToken {
    fn from(response: RawAuthResponse) -> BearerToken {
        BearerToken {
            access_token: response.access_token,
            expires_at: SystemTime::now()
                .checked_add(std::time::Duration::from_secs(response.expires_in))
                .unwrap(),
            refresh_token: response.refresh_token,
            scopes: response
                .scope
                .map(|scopes| {
                    scopes
                        .split(' ')
                        .filter_map(|s| Scope::try_from(s).ok())
                        .collect()
                })
                .unwrap_or_default(),
        }
    }
}

fn required_scopes() -> String {
    Scope::join(&[
        Scope::UserReadPlaybackState,
        Scope::UserModifyPlaybackState,
        Scope::UserReadCurrentlyPlaying,
        Scope::PlaylistModifyPrivate,
        Scope::PlaylistModifyPublic,
    ])
}

pub async fn request_user_credentials(state: &str) -> rocket::response::Redirect {
    let env = crate::env::env_vars();
    rocket::response::Redirect::to(uri!(
        "https://accounts.spotify.com",
        spotify_uri_bindings::authorize(
            client_id = &env.client_id,
            scope = required_scopes(),
            redirect_uri = &env.oauth_redirect_uri(),
            state = state,
        )
    ))
}

pub fn construct_basic_auth_header() -> String {
    let env = crate::env::env_vars();
    let auth_string = format!("{}:{}", env.client_id, env.client_secret);
    let auth_string = general_purpose::STANDARD.encode(auth_string);
    format!("Basic {}", auth_string)
}

pub async fn exchange_code_for_token(code: &str) -> Result<Token, Box<dyn std::error::Error>> {
    let request = reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", construct_basic_auth_header())
        .form(&[
            ("grant_type", "authorization_code"),
            ("code", code),
            ("redirect_uri", &crate::env::env_vars().oauth_redirect_uri()),
        ]);
    let raw_response: RawAuthResponse = request.send().await?.json().await?;
    Ok(raw_response.into())
}

pub async fn refresh_token(token: &mut BearerToken) -> Result<(), Box<dyn std::error::Error>> {
    if token.expires_at > SystemTime::now() {
        return Ok(());
    }
    force_refresh_token(token).await
}

pub async fn force_refresh_token(
    token: &mut BearerToken,
) -> Result<(), Box<dyn std::error::Error>> {
    let refresh_token = match &token.refresh_token {
        Some(rt) => rt,
        None => return Err("no refresh token available".into()),
    };
    let request = reqwest::Client::new()
        .post("https://accounts.spotify.com/api/token")
        .header("Content-Type", "application/x-www-form-urlencoded")
        .header("Authorization", construct_basic_auth_header())
        .form(&[
            ("grant_type", "refresh_token"),
            ("refresh_token", refresh_token),
        ]);
    let raw_response: RawAuthResponse = request.send().await?.json().await?;
    let new_token: BearerToken = raw_response.into();
    if !new_token.access_token.is_empty() {
        *token = new_token;
    }
    Ok(())
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum Scope {
    UGCImageUpload,
    UserReadPlaybackState,
    UserModifyPlaybackState,
    UserReadCurrentlyPlaying,
    AppRemoteControl,
    Streaming,
    PlaylistReadPrivate,
    PlaylistReadCollaborative,
    PlaylistModifyPrivate,
    PlaylistModifyPublic,
    UserFollowModify,
    UserFollowRead,
    UserReadPlaybackPosition,
    UserTopRead,
    UserReadRecentlyPlayed,
    UserLibraryModify,
    UserLibraryRead,
    UserReadEmail,
    UserReadPrivate,
    UserPersonalized,
    UserSOALink,
    UserSOAUnlink,
    SOAManageEntitlements,
    SOAManagePartner,
    SOACreatePartner,
}

impl Scope {
    pub fn join(scopes: &[Scope]) -> String {
        scopes
            .iter()
            .map(|s| s.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    }
}

impl Display for Scope {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        let scope_str = match self {
            Scope::UGCImageUpload => "ugc-image-upload",
            Scope::UserReadPlaybackState => "user-read-playback-state",
            Scope::UserModifyPlaybackState => "user-modify-playback-state",
            Scope::UserReadCurrentlyPlaying => "user-read-currently-playing",
            Scope::AppRemoteControl => "app-remote-control",
            Scope::Streaming => "streaming",
            Scope::PlaylistReadPrivate => "playlist-read-private",
            Scope::PlaylistReadCollaborative => "playlist-read-collaborative",
            Scope::PlaylistModifyPrivate => "playlist-modify-private",
            Scope::PlaylistModifyPublic => "playlist-modify-public",
            Scope::UserFollowModify => "user-follow-modify",
            Scope::UserFollowRead => "user-follow-read",
            Scope::UserReadPlaybackPosition => "user-read-playback-position",
            Scope::UserTopRead => "user-top-read",
            Scope::UserReadRecentlyPlayed => "user-read-recently-played",
            Scope::UserLibraryModify => "user-library-modify",
            Scope::UserLibraryRead => "user-library-read",
            Scope::UserReadEmail => "user-read-email",
            Scope::UserReadPrivate => "user-read-private",
            Scope::UserPersonalized => "user-personalized",
            Scope::UserSOALink => "user-soa-link",
            Scope::UserSOAUnlink => "user-soa-unlink",
            Scope::SOAManageEntitlements => "soa-manage-entitlements",
            Scope::SOAManagePartner => "soa-manage-partner",
            Scope::SOACreatePartner => "soa-create-partner",
        };
        write!(f, "{scope_str}")
    }
}

impl TryFrom<&str> for Scope {
    type Error = ();

    fn try_from(value: &str) -> Result<Self, Self::Error> {
        match value {
            "ugc-image-upload" => Ok(Scope::UGCImageUpload),
            "user-read-playback-state" => Ok(Scope::UserReadPlaybackState),
            "user-modify-playback-state" => Ok(Scope::UserModifyPlaybackState),
            "user-read-currently-playing" => Ok(Scope::UserReadCurrentlyPlaying),
            "app-remote-control" => Ok(Scope::AppRemoteControl),
            "streaming" => Ok(Scope::Streaming),
            "playlist-read-private" => Ok(Scope::PlaylistReadPrivate),
            "playlist-read-collaborative" => Ok(Scope::PlaylistReadCollaborative),
            "playlist-modify-private" => Ok(Scope::PlaylistModifyPrivate),
            "playlist-modify-public" => Ok(Scope::PlaylistModifyPublic),
            "user-follow-modify" => Ok(Scope::UserFollowModify),
            "user-follow-read" => Ok(Scope::UserFollowRead),
            "user-read-playback-position" => Ok(Scope::UserReadPlaybackPosition),
            "user-top-read" => Ok(Scope::UserTopRead),
            "user-read-recently-played" => Ok(Scope::UserReadRecentlyPlayed),
            "user-library-modify" => Ok(Scope::UserLibraryModify),
            "user-library-read" => Ok(Scope::UserLibraryRead),
            "user-read-email" => Ok(Scope::UserReadEmail),
            "user-read-private" => Ok(Scope::UserReadPrivate),
            "user-personalized" => Ok(Scope::UserPersonalized),
            "user-soa-link" => Ok(Scope::UserSOALink),
            "user-soa-unlink" => Ok(Scope::UserSOAUnlink),
            "soa-manage-entitlements" => Ok(Scope::SOAManageEntitlements),
            "soa-manage-partner" => Ok(Scope::SOAManagePartner),
            "soa-create-partner" => Ok(Scope::SOACreatePartner),
            _ => Err(()),
        }
    }
}
