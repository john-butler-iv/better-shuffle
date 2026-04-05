use std::time::{Duration, SystemTime};

use crate::core_types::{SpotifyURI, TrackID};

use super::{context::Context, device::Device, episode::Episode, track::Track};
use serde::de::Error;

#[derive(Debug, Clone)]
pub struct PlaybackState {
    //device: Device,
    //repeat_state: RepeatState,
    //is_shuffling: bool,
    context: Option<Context>,
    last_change_inst: SystemTime,
    progress: Option<Duration>,
    item: PlayingItem,
    allowed_actions: ActionSet,
}

impl PlaybackState {
    pub fn is_playing(&self) -> bool {
        self.item != PlayingItem::Nothing
    }

    pub fn is_playing_track(&self, track_id: &TrackID) -> bool {
        matches!(&self.item, PlayingItem::Track(track) if track == track_id)
    }

    pub fn uri(&self) -> Option<SpotifyURI> {
        match &self.item {
            PlayingItem::Track(track) => Some(track.uri()),
            PlayingItem::Episode(episode) => Some(episode.uri()),
            _ => None,
        }
    }

    pub fn time_remaining(&self) -> Option<Duration> {
        self.progress.and_then(|progress| {
            if let PlayingItem::Track(track) = &self.item {
                Some(*track.duration() - progress)
            } else {
                None
            }
        })
    }
}

impl<'de> serde::Deserialize<'de> for PlaybackState {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        RawPlaybackState::deserialize(deserializer)?
            .try_into()
            .map_err(|_| D::Error::custom("invalid inner playback state data"))
    }
}

#[derive(Debug, Clone, serde::Deserialize)]
struct RawPlaybackState {
    //device: Device,
    //repeat_state: RepeatState,
    //shuffle_state: bool,
    context: Option<Context>,
    timestamp: u64,
    progress_ms: Option<u64>,
    is_playing: bool,
    item: Option<Track>,
    currently_playing_type: String,
    actions: RawActionSetWrapper,
}

impl TryInto<PlaybackState> for RawPlaybackState {
    type Error = ();

    fn try_into(self) -> Result<PlaybackState, Self::Error> {
        let item: PlayingItem = if self.is_playing {
            match self.currently_playing_type.as_str() {
                "track" => PlayingItem::Track(self.item.unwrap()),
                "episode" => panic!("received an episode. Not handled"),
                "ad" => PlayingItem::Ad,
                _ => PlayingItem::Unknown,
            }
        } else {
            PlayingItem::Nothing
        };

        Ok(PlaybackState {
            //device: self.device,
            //repeat_state: self.repeat_state,
            //is_shuffling: self.shuffle_state,
            context: self.context,
            last_change_inst: SystemTime::UNIX_EPOCH + Duration::from_millis(self.timestamp),
            progress: self.progress_ms.map(Duration::from_millis),
            item,
            allowed_actions: self.actions.disallows,
        })
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum PlayingItem {
    Track(Track),
    Episode(Episode),
    Ad,
    Unknown,
    Nothing,
}

#[derive(Debug, Clone, Copy, serde::Serialize, serde::Deserialize, PartialEq, Eq)]
pub enum RepeatState {
    #[serde(rename = "off")]
    Off,
    #[serde(rename = "track")]
    Track,
    #[serde(rename = "context")]
    Context,
}

#[derive(Debug, Clone, PartialEq, serde::Deserialize, Eq)]
pub struct ActionSet {
    interrupting_playback: Option<bool>,
    pausing: Option<bool>,
    resuming: Option<bool>,
    seeking: Option<bool>,
    skipping_next: Option<bool>,
    skipping_previous: Option<bool>,
    toggling_repeat_context: Option<bool>,
    toggling_shuffle: Option<bool>,
    toggling_repeat_track: Option<bool>,
    transferring_playback: Option<bool>,
}

#[derive(Debug, Clone, serde::Deserialize)]
struct RawActionSetWrapper {
    disallows: ActionSet,
}

impl From<RawActionSetWrapper> for ActionSet {
    fn from(from: RawActionSetWrapper) -> Self {
        ActionSet {
            interrupting_playback: from.disallows.interrupting_playback.map(|a| !a),
            pausing: from.disallows.pausing.map(|a| !a),
            resuming: from.disallows.resuming.map(|a| !a),
            seeking: from.disallows.seeking.map(|a| !a),
            skipping_next: from.disallows.skipping_next.map(|a| !a),
            skipping_previous: from.disallows.skipping_previous.map(|a| !a),
            toggling_repeat_context: from.disallows.toggling_repeat_context.map(|a| !a),
            toggling_shuffle: from.disallows.toggling_shuffle.map(|a| !a),
            toggling_repeat_track: from.disallows.toggling_repeat_track.map(|a| !a),
            transferring_playback: from.disallows.transferring_playback.map(|a| !a),
        }
    }
}
