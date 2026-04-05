use crate::auth::Token;
use crate::core_types::device::Device;
use crate::core_types::playback_state::PlaybackState;
use crate::core_types::{DeviceID, PlaylistID};
use crate::core_types::{TrackID, track::Track};

pub async fn get_track(track_id: &TrackID, token: &mut Token) -> Result<Track, ()> {
    reqwest::Client::new()
        .get(format!("https://api.spotify.com/v1/tracks/{}", track_id))
        .bearer_auth(
            token
                .auth_header()
                .await
                .map_err(|err| eprintln!("{err:?}"))?,
        )
        .send()
        .await
        .map_err(|err| {
            eprintln!("{err:?}");
        })?
        .json()
        .await
        .map_err(|err| {
            eprintln!("{err:?}");
        })
}

pub async fn get_current_song(token: &mut Token) -> Result<Option<PlaybackState>, ()> {
    let headers = reqwest::Client::new()
        .get("https://api.spotify.com/v1/me/player/currently-playing")
        .bearer_auth(token.auth_header().await.map_err(|err| {
            eprintln!("{err:?}");
        })?)
        .send()
        .await
        .map_err(|err| {
            eprintln!("{err:?}");
        })?;
    if headers.status() == 204 {
        Ok(None)
    } else {
        headers.json().await.map_err(|err| {
            eprintln!("{err:?}");
        })

        //dbg!(headers.text().await);
        //Err(())
        /*
                {
                \n  \"is_playing\" : false,
                \n  \"timestamp\" : 1775406349661,
                \n  \"context\" : null,
                \n  \"progress_ms\" : 20526,
                \n  \"item\" : {
                \n    \"album\" : {
                \n      \"album_type\" : \"single\",
                \n      \"artists\" : [ {
                \n        \"external_urls\" : {
                \n          \"spotify\" : \"https://open.spotify.com/artist/6mWAKV1AAFvzxQr7uztRE9\"
                \n        },
                \n        \"href\" : \"https://api.spotify.com/v1/artists/6mWAKV1AAFvzxQr7uztRE9\",
                \n        \"id\" : \"6mWAKV1AAFvzxQr7uztRE9\",
                \n        \"name\" : \"Rob Araujo\",
                \n        \"type\" : \"artist\",
                \n        \"uri\" : \"spotify:artist:6mWAKV1AAFvzxQr7uztRE9\"
                \n      } ],
                \n      \"available_markets\" : [ \"AR\", \"AU\", \"AT\", \"BE\", \"BO\", \"BR\", \"BG\", \"CA\", \"CL\", \"CO\", \"CR\", \"CY\", \"CZ\", \"DK\", \"DO\", \"DE\", \"EC\", \"EE\", \"SV\", \"FI\", \"FR\", \"GR\", \"GT\", \"HN\", \"HK\", \"HU\", \"IS\", \"IE\", \"IT\", \"LV\", \"LT\", \"LU\", \"MY\", \"MT\", \"MX\", \"NL\", \"NZ\", \"NI\", \"NO\", \"PA\", \"PY\", \"PE\", \"PH\", \"PL\", \"PT\", \"SG\", \"SK\", \"ES\", \"SE\", \"CH\", \"TW\", \"TR\", \"UY\", \"US\", \"GB\", \"AD\", \"LI\", \"MC\", \"ID\", \"JP\", \"TH\", \"VN\", \"RO\", \"IL\", \"ZA\", \"SA\", \"AE\", \"BH\", \"QA\", \"OM\", \"KW\", \"EG\", \"MA\", \"DZ\", \"TN\", \"LB\", \"JO\", \"PS\", \"IN\", \"BY\", \"KZ\", \"MD\", \"UA\", \"AL\", \"BA\", \"HR\", \"ME\", \"MK\", \"RS\", \"SI\", \"KR\", \"BD\", \"PK\", \"LK\", \"GH\", \"KE\", \"NG\", \"TZ\", \"UG\", \"AG\", \"AM\", \"BS\", \"BB\", \"BZ\", \"BT\", \"BW\", \"BF\", \"CV\", \"CW\", \"DM\", \"FJ\", \"GM\", \"GE\", \"GD\", \"GW\", \"GY\", \"HT\", \"JM\", \"KI\", \"LS\", \"LR\", \"MW\", \"MV\", \"ML\", \"MH\", \"FM\", \"NA\", \"NR\", \"NE\", \"PW\", \"PG\", \"PR\", \"WS\", \"SM\", \"ST\", \"SN\", \"SC\", \"SL\", \"SB\", \"KN\", \"LC\", \"VC\", \"SR\", \"TL\", \"TO\", \"TT\", \"TV\", \"VU\", \"AZ\", \"BN\", \"BI\", \"KH\", \"CM\", \"TD\", \"KM\", \"GQ\", \"SZ\", \"GA\", \"GN\", \"KG\", \"LA\", \"MO\", \"MR\", \"MN\", \"NP\", \"RW\", \"TG\", \"UZ\", \"ZW\", \"BJ\", \"MG\", \"MU\", \"MZ\", \"AO\", \"CI\", \"DJ\", \"ZM\", \"CD\", \"CG\", \"IQ\", \"LY\", \"TJ\", \"VE\", \"ET\", \"XK\" ],
                \n      \"external_urls\" : {
                \n        \"spotify\" : \"https://open.spotify.com/album/5CT7ZFB3hy5XSxtMQnEoUv\"
                \n      },
                \n      \"href\" : \"https://api.spotify.com/v1/albums/5CT7ZFB3hy5XSxtMQnEoUv\",
                \n      \"id\" : \"5CT7ZFB3hy5XSxtMQnEoUv\",
                \n      \"images\" : [ {
                \n        \"height\" : 640,
                \n        \"url\" : \"https://i.scdn.co/image/ab67616d0000b273a8b020a3caa21028ff15df07\",
                \n        \"width\" : 640
                \n      }, {
                \n        \"height\" : 300,
                \n        \"url\" : \"https://i.scdn.co/image/ab67616d00001e02a8b020a3caa21028ff15df07\",
                \n        \"width\" : 300
                \n      }, {
                \n        \"height\" : 64,
                \n        \"url\" : \"https://i.scdn.co/image/ab67616d00004851a8b020a3caa21028ff15df07\",
                \n        \"width\" : 64
                \n      } ],
                \n      \"name\" : \"Nineteen\",
                \n      \"release_date\" : \"2018-10-12\",
                \n      \"release_date_precision\" : \"day\",
                \n      \"total_tracks\" : 1,
                \n      \"type\" : \"album\",
                \n      \"uri\" : \"spotify:album:5CT7ZFB3hy5XSxtMQnEoUv\"
                \n    },
                \n    \"artists\" : [ {
                \n      \"external_urls\" : {
                \n        \"spotify\" : \"https://open.spotify.com/artist/6mWAKV1AAFvzxQr7uztRE9\"
                \n      },
                \n      \"href\" : \"https://api.spotify.com/v1/artists/6mWAKV1AAFvzxQr7uztRE9\",
                \n      \"id\" : \"6mWAKV1AAFvzxQr7uztRE9\",
                \n      \"name\" : \"Rob Araujo\",
                \n      \"type\" : \"artist\",
                \n      \"uri\" : \"spotify:artist:6mWAKV1AAFvzxQr7uztRE9\"
                \n    } ],
                \n    \"available_markets\" : [ \"AR\", \"AU\", \"AT\", \"BE\", \"BO\", \"BR\", \"BG\", \"CA\", \"CL\", \"CO\", \"CR\", \"CY\", \"CZ\", \"DK\", \"DO\", \"DE\", \"EC\", \"EE\", \"SV\", \"FI\", \"FR\", \"GR\", \"GT\", \"HN\", \"HK\", \"HU\", \"IS\", \"IE\", \"IT\", \"LV\", \"LT\", \"LU\", \"MY\", \"MT\", \"MX\", \"NL\", \"NZ\", \"NI\", \"NO\", \"PA\", \"PY\", \"PE\", \"PH\", \"PL\", \"PT\", \"SG\", \"SK\", \"ES\", \"SE\", \"CH\", \"TW\", \"TR\", \"UY\", \"US\", \"GB\", \"AD\", \"LI\", \"MC\", \"ID\", \"JP\", \"TH\", \"VN\", \"RO\", \"IL\", \"ZA\", \"SA\", \"AE\", \"BH\", \"QA\", \"OM\", \"KW\", \"EG\", \"MA\", \"DZ\", \"TN\", \"LB\", \"JO\", \"PS\", \"IN\", \"BY\", \"KZ\", \"MD\", \"UA\", \"AL\", \"BA\", \"HR\", \"ME\", \"MK\", \"RS\", \"SI\", \"KR\", \"BD\", \"PK\", \"LK\", \"GH\", \"KE\", \"NG\", \"TZ\", \"UG\", \"AG\", \"AM\", \"BS\", \"BB\", \"BZ\", \"BT\", \"BW\", \"BF\", \"CV\", \"CW\", \"DM\", \"FJ\", \"GM\", \"GE\", \"GD\", \"GW\", \"GY\", \"HT\", \"JM\", \"KI\", \"LS\", \"LR\", \"MW\", \"MV\", \"ML\", \"MH\", \"FM\", \"NA\", \"NR\", \"NE\", \"PW\", \"PG\", \"PR\", \"WS\", \"SM\", \"ST\", \"SN\", \"SC\", \"SL\", \"SB\", \"KN\", \"LC\", \"VC\", \"SR\", \"TL\", \"TO\", \"TT\", \"TV\", \"VU\", \"AZ\", \"BN\", \"BI\", \"KH\", \"CM\", \"TD\", \"KM\", \"GQ\", \"SZ\", \"GA\", \"GN\", \"KG\", \"LA\", \"MO\", \"MR\", \"MN\", \"NP\", \"RW\", \"TG\", \"UZ\", \"ZW\", \"BJ\", \"MG\", \"MU\", \"MZ\", \"AO\", \"CI\", \"DJ\", \"ZM\", \"CD\", \"CG\", \"IQ\", \"LY\", \"TJ\", \"VE\", \"ET\", \"XK\" ],
                \n    \"disc_number\" : 1,
                \n    \"duration_ms\" : 283200,
                \n    \"explicit\" : false,
                \n    \"external_ids\" : {
                \n      \"isrc\" : \"QM24S1836131\"
                \n    },
                \n    \"external_urls\" : {
                \n      \"spotify\" : \"https://open.spotify.com/track/0LUgP4CrBbus8Yoq1C45pB\"
                \n    },
                \n    \"href\" : \"https://api.spotify.com/v1/tracks/0LUgP4CrBbus8Yoq1C45pB\",
                \n    \"id\" : \"0LUgP4CrBbus8Yoq1C45pB\",
                \n    \"is_local\" : false,
                \n    \"name\" : \"Nineteen\",
                \n    \"popularity\" : 53,
                \n    \"preview_url\" : null,
                \n    \"track_number\" : 1,
                \n    \"type\" : \"track\",
                \n    \"uri\" : \"spotify:track:0LUgP4CrBbus8Yoq1C45pB\"
                \n  },
                \n  \"currently_playing_type\" : \"track\",
                \n  \"actions\" : {
                \n    \"disallows\" : {
                \n      \"pausing\" : true,
                \n      \"skipping_prev\" : true
                \n    }
                \n  }
                \n}",
)
[src/spotify_actions.rs:42:9] headers.text().await = Ok(
    "{\n  \"is_playing\" : false,\n  \"timestamp\" : 1775406349661,\n  \"context\" : null,\n  \"progress_ms\" : 20526,\n  \"item\" : {\n    \"album\" : {\n      \"album_type\" : \"single\",\n      \"artists\" : [ {\n        \"external_urls\" : {\n          \"spotify\" : \"https://open.spotify.com/artist/6mWAKV1AAFvzxQr7uztRE9\"\n        },\n        \"href\" : \"https://api.spotify.com/v1/artists/6mWAKV1AAFvzxQr7uztRE9\",\n        \"id\" : \"6mWAKV1AAFvzxQr7uztRE9\",\n        \"name\" : \"Rob Araujo\",\n        \"type\" : \"artist\",\n        \"uri\" : \"spotify:artist:6mWAKV1AAFvzxQr7uztRE9\"\n      } ],\n      \"available_markets\" : [ \"AR\", \"AU\", \"AT\", \"BE\", \"BO\", \"BR\", \"BG\", \"CA\", \"CL\", \"CO\", \"CR\", \"CY\", \"CZ\", \"DK\", \"DO\", \"DE\", \"EC\", \"EE\", \"SV\", \"FI\", \"FR\", \"GR\", \"GT\", \"HN\", \"HK\", \"HU\", \"IS\", \"IE\", \"IT\", \"LV\", \"LT\", \"LU\", \"MY\", \"MT\", \"MX\", \"NL\", \"NZ\", \"NI\", \"NO\", \"PA\", \"PY\", \"PE\", \"PH\", \"PL\", \"PT\", \"SG\", \"SK\", \"ES\", \"SE\", \"CH\", \"TW\", \"TR\", \"UY\", \"US\", \"GB\", \"AD\", \"LI\", \"MC\", \"ID\", \"JP\", \"TH\", \"VN\", \"RO\", \"IL\", \"ZA\", \"SA\", \"AE\", \"BH\", \"QA\", \"OM\", \"KW\", \"EG\", \"MA\", \"DZ\", \"TN\", \"LB\", \"JO\", \"PS\", \"IN\", \"BY\", \"KZ\", \"MD\", \"UA\", \"AL\", \"BA\", \"HR\", \"ME\", \"MK\", \"RS\", \"SI\", \"KR\", \"BD\", \"PK\", \"LK\", \"GH\", \"KE\", \"NG\", \"TZ\", \"UG\", \"AG\", \"AM\", \"BS\", \"BB\", \"BZ\", \"BT\", \"BW\", \"BF\", \"CV\", \"CW\", \"DM\", \"FJ\", \"GM\", \"GE\", \"GD\", \"GW\", \"GY\", \"HT\", \"JM\", \"KI\", \"LS\", \"LR\", \"MW\", \"MV\", \"ML\", \"MH\", \"FM\", \"NA\", \"NR\", \"NE\", \"PW\", \"PG\", \"PR\", \"WS\", \"SM\", \"ST\", \"SN\", \"SC\", \"SL\", \"SB\", \"KN\", \"LC\", \"VC\", \"SR\", \"TL\", \"TO\", \"TT\", \"TV\", \"VU\", \"AZ\", \"BN\", \"BI\", \"KH\", \"CM\", \"TD\", \"KM\", \"GQ\", \"SZ\", \"GA\", \"GN\", \"KG\", \"LA\", \"MO\", \"MR\", \"MN\", \"NP\", \"RW\", \"TG\", \"UZ\", \"ZW\", \"BJ\", \"MG\", \"MU\", \"MZ\", \"AO\", \"CI\", \"DJ\", \"ZM\", \"CD\", \"CG\", \"IQ\", \"LY\", \"TJ\", \"VE\", \"ET\", \"XK\" ],\n      \"external_urls\" : {\n        \"spotify\" : \"https://open.spotify.com/album/5CT7ZFB3hy5XSxtMQnEoUv\"\n      },\n      \"href\" : \"https://api.spotify.com/v1/albums/5CT7ZFB3hy5XSxtMQnEoUv\",\n      \"id\" : \"5CT7ZFB3hy5XSxtMQnEoUv\",\n      \"images\" : [ {\n        \"height\" : 640,\n        \"url\" : \"https://i.scdn.co/image/ab67616d0000b273a8b020a3caa21028ff15df07\",\n        \"width\" : 640\n      }, {\n        \"height\" : 300,\n        \"url\" : \"https://i.scdn.co/image/ab67616d00001e02a8b020a3caa21028ff15df07\",\n        \"width\" : 300\n      }, {\n        \"height\" : 64,\n        \"url\" : \"https://i.scdn.co/image/ab67616d00004851a8b020a3caa21028ff15df07\",\n        \"width\" : 64\n      } ],\n      \"name\" : \"Nineteen\",\n      \"release_date\" : \"2018-10-12\",\n      \"release_date_precision\" : \"day\",\n      \"total_tracks\" : 1,\n      \"type\" : \"album\",\n      \"uri\" : \"spotify:album:5CT7ZFB3hy5XSxtMQnEoUv\"\n    },\n    \"artists\" : [ {\n      \"external_urls\" : {\n        \"spotify\" : \"https://open.spotify.com/artist/6mWAKV1AAFvzxQr7uztRE9\"\n      },\n      \"href\" : \"https://api.spotify.com/v1/artists/6mWAKV1AAFvzxQr7uztRE9\",\n      \"id\" : \"6mWAKV1AAFvzxQr7uztRE9\",\n      \"name\" : \"Rob Araujo\",\n      \"type\" : \"artist\",\n      \"uri\" : \"spotify:artist:6mWAKV1AAFvzxQr7uztRE9\"\n    } ],\n    \"available_markets\" : [ \"AR\", \"AU\", \"AT\", \"BE\", \"BO\", \"BR\", \"BG\", \"CA\", \"CL\", \"CO\", \"CR\", \"CY\", \"CZ\", \"DK\", \"DO\", \"DE\", \"EC\", \"EE\", \"SV\", \"FI\", \"FR\", \"GR\", \"GT\", \"HN\", \"HK\", \"HU\", \"IS\", \"IE\", \"IT\", \"LV\", \"LT\", \"LU\", \"MY\", \"MT\", \"MX\", \"NL\", \"NZ\", \"NI\", \"NO\", \"PA\", \"PY\", \"PE\", \"PH\", \"PL\", \"PT\", \"SG\", \"SK\", \"ES\", \"SE\", \"CH\", \"TW\", \"TR\", \"UY\", \"US\", \"GB\", \"AD\", \"LI\", \"MC\", \"ID\", \"JP\", \"TH\", \"VN\", \"RO\", \"IL\", \"ZA\", \"SA\", \"AE\", \"BH\", \"QA\", \"OM\", \"KW\", \"EG\", \"MA\", \"DZ\", \"TN\", \"LB\", \"JO\", \"PS\", \"IN\", \"BY\", \"KZ\", \"MD\", \"UA\", \"AL\", \"BA\", \"HR\", \"ME\", \"MK\", \"RS\", \"SI\", \"KR\", \"BD\", \"PK\", \"LK\", \"GH\", \"KE\", \"NG\", \"TZ\", \"UG\", \"AG\", \"AM\", \"BS\", \"BB\", \"BZ\", \"BT\", \"BW\", \"BF\", \"CV\", \"CW\", \"DM\", \"FJ\", \"GM\", \"GE\", \"GD\", \"GW\", \"GY\", \"HT\", \"JM\", \"KI\", \"LS\", \"LR\", \"MW\", \"MV\", \"ML\", \"MH\", \"FM\", \"NA\", \"NR\", \"NE\", \"PW\", \"PG\", \"PR\", \"WS\", \"SM\", \"ST\", \"SN\", \"SC\", \"SL\", \"SB\", \"KN\", \"LC\", \"VC\", \"SR\", \"TL\", \"TO\", \"TT\", \"TV\", \"VU\", \"AZ\", \"BN\", \"BI\", \"KH\", \"CM\", \"TD\", \"KM\", \"GQ\", \"SZ\", \"GA\", \"GN\", \"KG\", \"LA\", \"MO\", \"MR\", \"MN\", \"NP\", \"RW\", \"TG\", \"UZ\", \"ZW\", \"BJ\", \"MG\", \"MU\", \"MZ\", \"AO\", \"CI\", \"DJ\", \"ZM\", \"CD\", \"CG\", \"IQ\", \"LY\", \"TJ\", \"VE\", \"ET\", \"XK\" ],\n    \"disc_number\" : 1,\n    \"duration_ms\" : 283200,\n    \"explicit\" : false,\n    \"external_ids\" : {\n      \"isrc\" : \"QM24S1836131\"\n    },\n    \"external_urls\" : {\n      \"spotify\" : \"https://open.spotify.com/track/0LUgP4CrBbus8Yoq1C45pB\"\n    },\n    \"href\" : \"https://api.spotify.com/v1/tracks/0LUgP4CrBbus8Yoq1C45pB\",\n    \"id\" : \"0LUgP4CrBbus8Yoq1C45pB\",\n    \"is_local\" : false,\n    \"name\" : \"Nineteen\",\n    \"popularity\" : 53,\n    \"preview_url\" : null,\n    \"track_number\" : 1,\n    \"type\" : \"track\",\n    \"uri\" : \"spotify:track:0LUgP4CrBbus8Yoq1C45pB\"\n  },\n  \"currently_playing_type\" : \"track\",\n  \"actions\" : {\n    \"disallows\" : {\n      \"pausing\" : true,\n      \"skipping_prev\" : true\n    }\n  }\n}",         */
        /*
            headers.json().await.map(Some).map_err(|err| {
                eprintln!("{err:?}");
            })
        */
    }
}

pub async fn lookup_queue(token: &mut Token) -> Result<SpotifyQueue, ()> {
    reqwest::Client::new()
        .get("https://api.spotify.com/v1/me/player/queue")
        .bearer_auth(
            token
                .auth_header()
                .await
                .map_err(|err| eprintln!("{err:?}"))?,
        )
        .send()
        .await
        .map_err(|_| ())?
        .json()
        .await
        .map_err(|_| ())
}

#[derive(Clone, Debug, serde::Deserialize)]
pub struct SpotifyQueue {
    pub currently_playing: Option<Track>,
    pub queue: Vec<Track>,
}

pub async fn add_to_queue(track_id: &TrackID, token: &mut Token) -> Result<(), ()> {
    reqwest::Client::new()
        .post(format!(
            "https://api.spotify.com/v1/me/player/queue?uri=spotify:track:{track_id}",
        ))
        .bearer_auth(
            token
                .auth_header()
                .await
                .map_err(|err| eprintln!("{err:?}"))?,
        )
        .send()
        .await
        .map(|_| ())
        .map_err(|_| ())
}

pub async fn begin_playback(
    queue: impl IntoIterator<Item = &TrackID>,
    device: &DeviceID,
    token: &mut Token,
) -> Result<(), ()> {
    reqwest::Client::new()
        .put("https://api.spotify.com/v1/me/player/play")
        .bearer_auth(
            token
                .auth_header()
                .await
                .map_err(|err| eprintln!("{err:?}"))?,
        )
        .query(&[["device_id", &format!("{device}")]])
        .json(&serde_json::json!({
            "uris": queue.into_iter().map(|id| format!("spotify:track:{id}")).collect::<Vec<_>>(),
            "position_ms": 0,
        }))
        .send()
        .await
        .map(|_| ())
        .map_err(|err| {
            eprintln!("{err:?}");
        })
}

pub async fn lookup_playlist_contents(
    playlist: &PlaylistID,
    token: &mut Token,
) -> Result<Vec<TrackID>, ()> {
    let mut track_ids = Vec::new();
    let mut url = format!(
        "https://api.spotify.com/v1/playlists/{playlist}/items?fields=items.item(id,is_playable,name),next&limit=50"
    );
    while !url.is_empty() {
        let page: PlaylistPage = reqwest::Client::new()
            .get(&url)
            .bearer_auth(token.auth_header().await.map_err(|err| {
                eprintln!("{err:?}");
            })?)
            .send()
            .await
            .map_err(|err| {
                eprintln!("{err:?}");
            })?
            .json()
            .await
            .map_err(|err| {
                eprintln!("{err:?}");
            })?;

        track_ids.extend(page.items.into_iter().filter_map(|item| {
            if item.item.id.is_none() && item.item.is_playable.unwrap_or(false) {
                eprintln!("skipping playable item with no track id: {:?}", item.item);
            }
            item.item.id
        }));

        url = page.next.unwrap_or_default();
    }

    Ok(track_ids)
}

#[derive(Clone, Debug, serde::Deserialize)]
struct PlaylistPage {
    items: Vec<TrackWrapper>,
    next: Option<String>,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct TrackWrapper {
    item: TrackIDWrapper,
}

#[derive(Clone, Debug, serde::Deserialize)]
struct TrackIDWrapper {
    id: Option<TrackID>,
    is_playable: Option<bool>,
    #[allow(dead_code)] // this is used to log when we skip a track that doesn't have an ID
    name: String,
}

pub async fn get_device_list(token: &mut Token) -> Result<Vec<Device>, ()> {
    let devices: DeviceListWrapper = reqwest::Client::new()
        .get("https://api.spotify.com/v1/me/player/devices")
        .bearer_auth(token.auth_header().await.map_err(|err| {
            eprintln!("{err:?}");
        })?)
        .send()
        .await
        .map_err(|err| {
            eprintln!("{err:?}");
        })?
        .json()
        .await
        .map_err(|err| {
            eprintln!("{err:?}");
        })?;
    Ok(devices.devices)
}

#[derive(Debug, Clone, serde::Deserialize)]
struct DeviceListWrapper {
    devices: Vec<Device>,
}

pub async fn find_preferred_device(token: &mut Token) -> Option<Device> {
    let devices = get_device_list(token).await.ok()?;

    let mut preferred_device = None;

    for device in devices.into_iter() {
        if is_new_device_preferred(preferred_device.as_ref(), &device) {
            preferred_device = Some(device);
        }
    }

    preferred_device
}

#[allow(clippy::if_same_then_else)]
fn is_new_device_preferred(old_device: Option<&Device>, new_device: &Device) -> bool {
    if let Some(old_device) = old_device {
        if old_device.is_active() {
            return false;
        } else if new_device.is_active() {
            return true;
        } else if old_device.name().contains("iPhone") {
            return false;
        } else if new_device.name().contains("iPhone") {
            return false;
        } else if old_device.name() == "John's MacBook Air" {
            return true;
        } else if new_device.name() == "John's MacBook Air" {
            return false;
        } else if old_device.id().is_some() {
            return false;
        } else if new_device.id().is_some() {
            return true;
        }
    } else {
        return true;
    }
    false
}
