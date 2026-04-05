//
//  ShuffleEntry.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/26/26.
//

import Foundation
import DequeModule

struct ShuffleEntry {
    static let SECONDS_PER_DAY = 24 * 60 * 60;
    static let RECENTS_LOOKBACK_PERIOD = Double(14 * SECONDS_PER_DAY);
    
    public let trackID: TrackID;
    var uri: SpotifyURI {
        .Track(self.trackID)
    }
    var track: Track?;
    var recentManualPlays: Array<Date>;
    var recentAutomaticPlays: Array<Date>;
    var recentSkips: Array<Date>;
    
    init(trackID: TrackID){
        self.trackID = trackID;
        self.track = nil;
        self.recentManualPlays = Array<Date>();
        self.recentAutomaticPlays = Array<Date>();
        self.recentSkips = Array<Date>();
    }
    
    public mutating func track(token: inout Token) async -> Track {
        if let track = self.track {
            return track;
        }
        let track = await lookupTrack(trackID: self.trackID, token: &token);
        self.track = track;
        return track;
    }
    
    mutating func trimRecents() {
        ShuffleEntry.trimSpecificRecent(recent:  &self.recentManualPlays);
        ShuffleEntry.trimSpecificRecent(recent: &self.recentAutomaticPlays);
        ShuffleEntry.trimSpecificRecent(recent: &self.recentSkips);
    }
    static func trimSpecificRecent(recent: inout Array<Date>) {
        recent = recent.filter({date in
            date.distance(to: Date.now) < RECENTS_LOOKBACK_PERIOD
        })
    }
    
    mutating func logRecent() {
        logRecent(date: Date.now)
    }
    mutating func logRecent(date: Date){
        recentAutomaticPlays.append(date)
    }
    
    mutating func logManualPlay() {
        logManualPlay(date: Date.now)
    }
    mutating func logManualPlay(date: Date){
        recentAutomaticPlays.append(date);
    }
    
    mutating func logSkip() {
        logSkip(date: Date.now)
    }
    mutating func logSkip(date: Date){
        recentSkips.append(date);
    }
    
    var score: Int {
        self.recentManualPlays.count - self.recentAutomaticPlays.count - 2 * self.recentSkips.count
    }
}

