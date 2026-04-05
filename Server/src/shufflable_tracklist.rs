mod shuffle_entry;

use shuffle_entry::ShuffleEntry;

use std::{
    cmp,
    collections::{BinaryHeap, HashMap, VecDeque},
    fs::{self, File},
    io::Write,
    thread::sleep,
    time::Duration,
};

use crate::{
    auth::Token,
    core_types::{DeviceID, context::ContextID},
    env,
    spotify_actions::{self, get_device_list, lookup_playlist_contents},
};

const MINIMUM_QUEUE_SIZE: usize = 10;
const MAXIMUM_QUEUE_SIZE: usize = 15;
const POLLING_DURATION_SECONDS: Duration = Duration::from_secs(10);
const SKIP_GRACE_PERIOD_SECONDS: Duration = Duration::from_secs(5);

#[derive(Debug, Clone)]
pub struct ShufflableTracklist {
    remaining: BinaryHeap<ShuffleBucket>,
    queue: VecDeque<ShuffleEntry>,
    played: Vec<ShuffleEntry>,

    context: ContextID,
}
impl ShufflableTracklist {
    pub async fn fetch(context: &ContextID, token: &mut Token) -> Self {
        let mut tracklist = ShufflableTracklist {
            remaining: BinaryHeap::new(),
            queue: VecDeque::new(),
            played: Vec::new(),

            context: context.clone(),
        };
        tracklist.repopulate_remaining_from(Self::fetch_entries(context, token).await);
        tracklist
    }

    pub async fn fetch_entries(context: &ContextID, token: &mut Token) -> Vec<ShuffleEntry> {
        if let Some(tracklist) = Self::try_read_from_disk(context) {
            tracklist
        } else if let ContextID::Playlist(playlist) = context {
            lookup_playlist_contents(playlist, token)
                .await
                .unwrap_or_default()
                .into_iter()
                .map(ShuffleEntry::new)
                .collect()
        } else {
            panic!("non-playlist contexts aren't supported yet")
        }
    }

    pub fn try_read_from_disk(context: &ContextID) -> Option<Vec<ShuffleEntry>> {
        let file_path = format!("{}/{}.json", env::env_vars().playlist_data_path, context);
        match fs::read_to_string(file_path) {
            Ok(contents) => serde_json::from_str(&contents).ok(),
            Err(_) => None,
        }
    }

    pub fn write_to_disk(&self) -> () {
        let mut entries: Vec<ShuffleEntry> = Vec::new();
        entries.extend(self.played.iter_mut().map(|entry|{ 
            entry.trim_recents();
            entry.clone()
        }));
        entries.extend(self.queue.iter_mut().map(|entry|{ 
            entry.trim_recents();
            entry.clone()
        }));
        self.refresh_stale_data();
        entries.extend(self.remaining.iter_mut().flat_map(|bucket| bucket.0).iter().map(|entry| {
            let entry = entry.clone();
            entry.
        }));

        let contents = serde_json::to_string(&entries).unwrap();

        let file_path = format!(
            "{}/{}.json",
            env::env_vars().playlist_data_path,
            self.context
        );
        let mut file = File::create(file_path).expect("Could not create file!");

        file.write_all(contents.as_bytes())
            .expect("Cannot write to the file!");
    }

    fn refresh_stale_data(&mut self) {
        let mut songs_to_refresh = std::mem::take(&mut self.remaining);
        self.repopulate_remaining_from(songs_to_refresh.drain().flat_map(|bucket| bucket.0));
    }

    fn repopulate_remaining_from(&mut self, from: impl IntoIterator<Item = ShuffleEntry>) {
        let mut new_remaining: HashMap<u32, Vec<ShuffleEntry>> = HashMap::new();
        for mut entry in from.into_iter() {
            entry.trim_recents();
            if let Some(existing_bucket) = new_remaining.get_mut(&entry.score()) {
                existing_bucket.push(entry);
            } else {
                new_remaining.insert(entry.score(), vec![entry]);
            }
        }
        for (_, value) in new_remaining.into_iter() {
            self.remaining.push(ShuffleBucket(value));
        }
    }

    fn try_repopulate_remaining(&mut self) {
        if !self.remaining.is_empty() {
            return;
        }
        let played = std::mem::take(&mut self.played);
        self.repopulate_remaining_from(played);
    }

    fn pick_song(&mut self) -> ShuffleEntry {
        self.try_repopulate_remaining();

        let mut preferred_bucket = self.remaining.pop().expect("we always maintain the invariant that there is at least one bucket if there are any songs left");
        let preferred_song = preferred_bucket
            .0
            .pop()
            .expect("we always maintain the invariant that buckets are non-empty");
        if !preferred_bucket.0.is_empty() {
            self.remaining.push(preferred_bucket);
        }
        preferred_song
    }

    async fn populate_queue(
        &mut self,
        device: &DeviceID,
        token: &mut crate::auth::Token,
    ) -> Result<(), ()> {
        if self.queue.len() >= MINIMUM_QUEUE_SIZE {
            return Ok(());
        }

        let need_to_start_song = crate::spotify_actions::get_current_song(token)
            .await
            .map(|playback| {
                playback.is_none_or(|playback| !playback.is_playing()) || self.queue.is_empty()
            })
            .unwrap_or(false);
        while self.queue.len() < MAXIMUM_QUEUE_SIZE {
            let preferred_song = self.pick_song();
            self.queue.push_back(preferred_song);
            if !need_to_start_song {
                let song = self.queue.back().expect("we just added this song");
                spotify_actions::add_to_queue(&song.track_id, token)
                    .await
                    .map_err(|err| {
                        eprintln!("{err:?}");
                    })?;
            }
        }

        if need_to_start_song {
            println!("starting");
            spotify_actions::begin_playback(
                self.queue.iter().map(|entry| &entry.track_id),
                device,
                token,
            )
            .await
            .map_err(|err| eprintln!("{err:?}"))?;
        }

        Ok(())
    }

    pub async fn manage_playlist(&mut self, token: &mut crate::auth::Token) -> Result<(), ()> {
        let device = spotify_actions::find_preferred_device(token)
            .await
            .ok_or(())?;

        loop {
            self.populate_queue(device.id().unwrap(), token).await?;
            let mut current_song = self
                .queue
                .pop_front()
                .expect("queue was just populated, and the playlist isn't empty");

            current_song.log_automatic_play();

            let mut song_still_playing = true;
            let mut playback_state = spotify_actions::get_current_song(token).await?;
            let mut time_remaining = playback_state.as_ref().unwrap().time_remaining();
            while song_still_playing {
                time_remaining = playback_state.as_ref().unwrap().time_remaining();

                sleep(POLLING_DURATION_SECONDS);

                playback_state = spotify_actions::get_current_song(token).await?;
                if let Some(ref playback_state) = playback_state
                    && playback_state.is_playing()
                {
                    song_still_playing = playback_state.is_playing_track(&current_song.track_id);
                } else {
                    return Ok(());
                }
            }

            if time_remaining.unwrap() + SKIP_GRACE_PERIOD_SECONDS > POLLING_DURATION_SECONDS {
                current_song.log_skip();

                if let Some(ref playback_state) = playback_state {
                    while let Some(next_song) = self.queue.front()
                        && !playback_state.is_playing_track(&next_song.track_id)
                    {
                        let mut skipped_track = self.queue.pop_front().unwrap();
                        skipped_track.log_skip();
                        self.played.push(skipped_track);
                    }
                }
            }
            self.played.push(current_song);
        }
    }
}

#[derive(Debug, Clone)]
struct ShuffleBucket(Vec<ShuffleEntry>);
impl PartialEq for ShuffleBucket {
    fn eq(&self, other: &Self) -> bool {
        match (self.0.first(), other.0.first()) {
            (Some(self_rep), Some(other_rep)) => self_rep.score() == other_rep.score(),
            _ => true,
        }
    }
}
impl Eq for ShuffleBucket {}
// the Ord relies on invarients that I'm maintaining. It's possible I will choose to break those invarients in the future.
// If that happens, I can simply remove the Ord implemenation and keep PartialOrd as is.
#[allow(clippy::non_canonical_partial_ord_impl)]
impl PartialOrd for ShuffleBucket {
    fn partial_cmp(&self, other: &Self) -> Option<cmp::Ordering> {
        match (self.0.first(), other.0.first()) {
            (Some(self_rep), Some(other_rep)) => self_rep.score().partial_cmp(&other_rep.score()),
            _ => None,
        }
    }
}
impl Ord for ShuffleBucket {
    fn cmp(&self, other: &Self) -> cmp::Ordering {
        self.partial_cmp(other)
            .expect("all entry buckets are non-empty")
    }
}
