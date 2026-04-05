use std::time::{Duration, SystemTime};

use crate::{
    auth::Token,
    core_types::{TrackID, track::Track},
};

const RECENTS_MAX_COUNT: usize = 50;
const SECONDS_PER_DAY: u64 = 24 * 60 * 60;
const RECENTS_LOOKBACK_PERIOD: Duration = Duration::from_secs(30 * SECONDS_PER_DAY);

#[derive(Clone, Debug, Eq)]
pub struct ShuffleEntry {
    pub track_id: TrackID,
    track: Option<Track>,
    recent_manual_plays: [Option<SystemTime>; RECENTS_MAX_COUNT],
    recent_automatic_plays: [Option<SystemTime>; RECENTS_MAX_COUNT],
    recent_skips: [Option<SystemTime>; RECENTS_MAX_COUNT],
}

impl PartialEq for ShuffleEntry {
    fn eq(&self, other: &Self) -> bool {
        self.track_id == other.track_id
    }
}

impl serde::Serialize for ShuffleEntry {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: serde::Serializer,
    {
        SerializableShuffleEntry::from(self.clone()).serialize(serializer)
    }
}

impl<'de> serde::Deserialize<'de> for ShuffleEntry {
    fn deserialize<D>(deserializer: D) -> Result<Self, D::Error>
    where
        D: serde::Deserializer<'de>,
    {
        Ok(ShuffleEntry::from(SerializableShuffleEntry::deserialize(
            deserializer,
        )?))
    }
}

#[derive(Debug, Clone, serde::Serialize, serde::Deserialize)]
struct SerializableShuffleEntry {
    track_id: TrackID,
    recent_manual_plays: Vec<SystemTime>,
    recent_automatic_plays: Vec<SystemTime>,
    recent_skips: Vec<SystemTime>,
}

impl From<ShuffleEntry> for SerializableShuffleEntry {
    fn from(value: ShuffleEntry) -> Self {
        SerializableShuffleEntry {
            track_id: value.track_id,
            recent_manual_plays: value.recent_manual_plays.into_iter().flatten().collect(),
            recent_automatic_plays: value.recent_automatic_plays.into_iter().flatten().collect(),
            recent_skips: value.recent_skips.into_iter().flatten().collect(),
        }
    }
}

impl From<SerializableShuffleEntry> for ShuffleEntry {
    fn from(value: SerializableShuffleEntry) -> Self {
        let mut entry = ShuffleEntry {
            track_id: value.track_id,
            track: None,
            recent_manual_plays: [None; 50],
            recent_automatic_plays: [None; 50],
            recent_skips: [None; 50],
        };

        let threshold_date = SystemTime::now() - RECENTS_LOOKBACK_PERIOD;

        let mut entry_index = 0;
        for timestamp in value.recent_manual_plays.into_iter() {
            if timestamp <= threshold_date {
                continue;
            }
            entry.recent_manual_plays[entry_index] = Some(timestamp);
            entry_index += 1;
            if entry_index >= entry.recent_manual_plays.len() {
                break;
            }
        }

        entry_index = 0;
        for timestamp in value.recent_automatic_plays.into_iter() {
            if timestamp <= threshold_date {
                continue;
            }
            entry.recent_automatic_plays[entry_index] = Some(timestamp);
            entry_index += 1;
            if entry_index >= entry.recent_automatic_plays.len() {
                break;
            }
        }

        entry_index = 0;
        for timestamp in value.recent_skips.into_iter() {
            if timestamp <= threshold_date {
                continue;
            }
            entry.recent_skips[entry_index] = Some(timestamp);
            entry_index += 1;
            if entry_index >= entry.recent_skips.len() {
                break;
            }
        }

        entry
    }
}

impl ShuffleEntry {
    pub fn new(track_id: TrackID) -> Self {
        ShuffleEntry {
            track_id,
            track: None,
            recent_manual_plays: [None; RECENTS_MAX_COUNT],
            recent_automatic_plays: [None; RECENTS_MAX_COUNT],
            recent_skips: [None; RECENTS_MAX_COUNT],
        }
    }

    pub async fn track(&mut self, token: &mut Token) -> Result<&Track, ()> {
        if self.track.is_none() {
            self.track = Some(crate::spotify_actions::get_track(&self.track_id, token).await?);
        }

        let track = self.track.as_ref().expect("we just looked it up");
        Ok(track)
    }

    fn count_recents(recents: &[Option<SystemTime>; RECENTS_MAX_COUNT]) -> usize {
        recents.iter().filter(|inst| inst.is_some()).count()
    }

    pub fn trim_recents(&mut self) {
        Self::trim_specific_recents(&mut self.recent_automatic_plays);
        Self::trim_specific_recents(&mut self.recent_manual_plays);
        Self::trim_specific_recents(&mut self.recent_skips);
    }
    fn trim_specific_recents(recents: &mut [Option<SystemTime>; RECENTS_MAX_COUNT]) {
        let threshold_date = SystemTime::now() - RECENTS_LOOKBACK_PERIOD;
        let mut first_valid_index: Option<usize> = None;
        for (i, recent) in recents.iter().enumerate() {
            if let Some(date) = recent {
                if *date >= threshold_date {
                    first_valid_index = Some(i);
                }
            } else {
                break;
            }
        }

        if let Some(first_valid_index) = first_valid_index {
            let total_valid_indexes = recents.len() - first_valid_index;
            for i in 0..total_valid_indexes {
                recents[i] = recents[i + first_valid_index];
            }
            for recent in recents[total_valid_indexes..].iter_mut() {
                *recent = None;
            }
        } else {
            for recent in recents.iter_mut() {
                *recent = None;
            }
        }
    }

    fn log_recent(recents: &mut [Option<SystemTime>; RECENTS_MAX_COUNT], time_played: SystemTime) {
        for recent in recents.iter_mut() {
            if recent.is_none() {
                *recent = Some(time_played);
                return;
            }
        }
        recents.rotate_left(1);
        recents[recents.len() - 1] = Some(time_played);
    }

    pub fn log_manual_play(&mut self) {
        self.log_manual_play_at_time(SystemTime::now())
    }
    pub fn log_manual_play_at_time(&mut self, time_played: SystemTime) {
        Self::log_recent(&mut self.recent_manual_plays, time_played);
    }

    pub fn log_automatic_play(&mut self) {
        self.log_automatic_play_at_time(SystemTime::now())
    }
    pub fn log_automatic_play_at_time(&mut self, time_played: SystemTime) {
        Self::log_recent(&mut self.recent_automatic_plays, time_played);
    }

    pub fn log_skip(&mut self) {
        println!("detected skip {self:?}");
        self.log_skip_at_time(SystemTime::now())
    }
    pub fn log_skip_at_time(&mut self, time_played: SystemTime) {
        Self::log_recent(&mut self.recent_skips, time_played);
    }

    pub fn score(&self) -> u32 {
        (Self::count_recents(&self.recent_manual_plays)
            - Self::count_recents(&self.recent_automatic_plays)
            - 2 * Self::count_recents(&self.recent_skips)) as u32
    }
}
