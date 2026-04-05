//
//  ShufflableTracklist.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/26/26.
//

import Foundation
import DequeModule
import HeapModule

public struct ShufflableTracklist {
    static let POLLING_DURATION_SECONDS: UInt32 = 10;
    static let MINIMUM_QUEUE_SIZE = 10;
    static let MAXIMUM_QUEUE_SIZE = 15;
    static let SKIP_GRACE_PERIOD_SECONDS: Int64 = 5;
    var remaining: Heap<ShuffleBucket>
    var queue: Deque<ShuffleEntry>
    var played: Array<ShuffleEntry>
    
    let context: ContextType
    
    mutating func refreshStaleData() {
        let localRemaining = self.remaining;
        self.remaining = Heap();
        repopulateRemainingFrom(from: localRemaining.unordered.flatMap({ bucket in bucket.entries }));
    }
    
    mutating func repopulateRemainingFrom(from: some Sequence<ShuffleEntry>){
        var newRemaining: Dictionary<Int, Array<ShuffleEntry>> = Dictionary();
        for var entry in from {
            entry.trimRecents();
            if var existingBucket = newRemaining[entry.score] {
                existingBucket.append(entry);
            } else {
                newRemaining[entry.score] = [entry];
            }
        }
        for key in newRemaining.keys {
            self.remaining.insert(ShuffleBucket(newRemaining[key]!))
        }
    }
    
    mutating func tryRepopulateRemaining(){
        if !self.remaining.isEmpty {
            return;
        }
        
        repopulateRemainingFrom(from: self.played);
        self.played = [];
    }
    
    mutating func pickSong() -> ShuffleEntry {
        self.tryRepopulateRemaining();
        
        var preferredBucket = remaining.max;
        let preferredSong = preferredBucket!.entries.removeLast();
        if preferredBucket!.entries.isEmpty {
            _ = self.remaining.popMax();
        }
        return preferredSong;
    }
   
    mutating func populateQueue(token: inout Token) async {
        if self.queue.count >= Self.MINIMUM_QUEUE_SIZE { return; }
        guard let playbackState = await lookupCurrentTrack(token: &token) else { return; }
        
        var needToStartSong = !playbackState.isPlaying && self.queue.isEmpty;
        while self.queue.count < Self.MAXIMUM_QUEUE_SIZE {
            let preferredSong = self.pickSong();
            queue.append(preferredSong);
            if needToStartSong {
                await beginPlayback(firstSong: preferredSong.uri, contextId: self.context.uri, token: &token);
                needToStartSong = false;
            } else {
                await addToQueue(uri: preferredSong.uri, token: &token)
            }
        }
    }
    
    mutating func managePlaylist(token: inout Token) async {
        await self.populateQueue(token: &token);
        
        var nextSong = self.queue.popFirst()!;
        nextSong.logRecent();
        
        while true {
            guard let playbackState = await lookupCurrentTrack(token: &token) else { return; }
            var timeRemaining: TimeInterval;
            switch playbackState.item! {
            case .TrackItem(let track):
                timeRemaining = Double(track.duration.components.seconds) - playbackState.progress!;
            default:
                // TODO
                return;
            }
            
            sleep(Self.POLLING_DURATION_SECONDS);
            
            guard let playbackState = await lookupCurrentTrack(token: &token) else { return; }
            if !playbackState.isPlaying {
                return;
            }
            
            if playbackState.uri == nextSong.uri {
                continue;
            }
            
            
            
            let skipped = timeRemaining + Double(Self.SKIP_GRACE_PERIOD_SECONDS) > Double(Self.POLLING_DURATION_SECONDS);
            if skipped {
                nextSong.logSkip();
            }
            self.played.append(nextSong)
            
            await self.populateQueue(token: &token);
            nextSong = self.queue.popFirst()!;
            nextSong.logRecent();

        }
    }
}

struct ShuffleBucket: Comparable, Equatable {
    var entries: Array<ShuffleEntry>;
    init(_ entries: Array<ShuffleEntry>) {
        self.entries = entries;
    }
    init(entry: ShuffleEntry) {
        self.init([entry])
    }
    static func < (lhs: ShuffleBucket, rhs: ShuffleBucket) -> Bool {
        lhs.entries[0].score < rhs.entries[0].score
    }
    static func == (lhs: ShuffleBucket, rhs: ShuffleBucket) -> Bool {
        lhs.entries[0].score == rhs.entries[0].score
    }
}
