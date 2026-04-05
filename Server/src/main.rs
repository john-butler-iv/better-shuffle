use core_types::{PlaylistID, SpotifyID, SpotifyURI, context::ContextID};

use crate::auth::BearerToken;

mod auth;
mod core_types;
mod env;
mod shufflable_tracklist;
mod spotify_actions;
mod spotify_uri_bindings;

const DEBUG: bool = true;

#[macro_use]
extern crate rocket;

#[get("/begin_shuffle?<playlist>")]
async fn shuffle_playlist(playlist: String) -> Result<(), ()> {
    let playlist = if let Ok(uri) = SpotifyURI::try_from(playlist.as_ref()) {
        match uri {
            SpotifyURI::Playlist(playlist) => playlist,
            _ => return Err(()),
        }
    } else {
        PlaylistID(SpotifyID(playlist))
    };

    let mut token = auth::Token::Bearer(
        BearerToken::from_environment_refresh_token()
            .await
            .ok_or(())?,
    );
    let mut tracklist = shufflable_tracklist::ShufflableTracklist::fetch(
        &ContextID::Playlist(playlist),
        &mut token,
    )
    .await;

    //std::thread::spawn(async move || tracklist.manage_playlist(&mut token).await);
    tracklist.manage_playlist(&mut token).await?;

    Ok(())
}

#[post("/print_refreshed_token")]
async fn print_refreshed_token() -> Result<(), ()> {
    if let Some(token) =
        auth::BearerToken::from_refresh_token(env::env_vars().known_refresh_token.clone()).await
    {
        println!("refreshed token: {token:?}");
    }
    Ok(())
}

#[get("/?<code>&<state>")]
#[allow(unused_variables)]
async fn auth_callback(code: String, state: String) -> Result<(), ()> {
    let token = auth::exchange_code_for_token(&code).await.map_err(|_| ())?;

    println!("token: {token:?}");
    Ok(())
}

#[launch]
fn rocket() -> _ {
    let env = env::env_vars();
    rocket::build()
        .configure(rocket::Config::figment().merge(("port", env.redirect_port)))
        .mount(
            format!("/{}", env.oauth_redirect_path),
            routes![auth_callback],
        )
        .mount("/", routes![shuffle_playlist, print_refreshed_token])
    /*
    .manage(ServerState {
        map: Mutex::new(HashMap::new()),
    })
    */
}
