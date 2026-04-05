//
//  PlaybackState.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/28/26.
//

import Foundation

struct PlaybackState {
    let device: Device
    let repeatState: RepeatState
    let shuffleState: Bool
    let context: Context?
    let lastChange: Date
    let progress: TimeInterval?
    let isPlaying: Bool
    let item: PlayableItem?
    let allowedActions: AllowedActionSet
    
    
    init?(json: Dictionary<String, AnyObject>){
        guard let deviceJson = json["device"] as? Dictionary<String, AnyObject>,
              let device = Device(json: deviceJson),
              let repeateStateString = json["repeat"] as? String,
              let repeatState = RepeatState(string: repeateStateString),
              let shuffleState = json["shuffle"] as? Bool,
              let contextJson = json["context"] as? Dictionary<String, AnyObject>?,
              let lastChangeMSInst = json["timestamp"] as? Double,
              let progressMS = json["progress_ms"] as? Double?,
              let isPlaying = json["is_playing"] as? Bool,
              let itemJson = json["item"] as? Dictionary<String, AnyObject>?,
              let itemType = json["currently_playing_type"] as? String,
              let actionsJson = json["actions"] as? Dictionary<String, AnyObject>,
              let actions = AllowedActionSet(json: actionsJson)
        else {return nil;}
        
        self.device = device;
        self.repeatState = repeatState;
        self.shuffleState = shuffleState;
        if contextJson == nil {
            self.context = nil;
        } else {
            guard let context = Context(json: contextJson!) else {return nil;}
            self.context = context;
        }
        self.lastChange = Date(timeIntervalSince1970: lastChangeMSInst / 1000)
        if progressMS == nil {
            self.progress = nil;
        } else {
            self.progress = TimeInterval(progressMS! / 1000)
        }
        self.isPlaying = isPlaying
        if let itemJson = itemJson {
            switch itemType {
            case "track":
                guard let track = Track(json: itemJson) else {return nil;}
                self.item = .TrackItem(track)
            case "episode":
                guard let episode = Episode(json: itemJson) else {return nil;}
                self.item = .EpisodeItem(episode)
            case "ad": self.item = nil;
            case "unknown": self.item = nil;
            default: return nil;
            }
        } else {
            self.item = nil;
        }
        self.allowedActions = actions;
    }
    
    
    var uri: SpotifyURI? {
        switch item {
        case .TrackItem(let track): return track.uri;
        case .EpisodeItem(let episode): return episode.uri;
        case nil: return nil;
        }
    }
}

enum RepeatState {
    case Off
    case Track
    case Context
    
    init?(string: String){
        switch string {
        case "off": self = .Off;
        case "track": self = .Track;
        case "context": self = .Context;
        default: return nil;
        }
    }
}

struct AllowedActionSet {
    let interruptPlayback: Bool?
    let pause: Bool?
    let resume: Bool?
    let seek: Bool?
    let skipNext: Bool?
    let skipPrevious: Bool?
    let toggleRepeatContext: Bool?
    let toggleShuffle: Bool?
    let toggleRepeatTrack: Bool?
    let transferPlayback: Bool?
    
    init?(json: Dictionary<String, AnyObject>){
        guard let interruptPlayback = json["interrupting_playback"] as? Bool?,
              let pause = json["pausing"] as? Bool?,
              let resume = json["resuming"] as? Bool?,
              let seek = json["seeking"] as? Bool?,
              let skipNext = json["skipping_next"] as? Bool?,
              let skipPrevious = json["skipping_prev"] as? Bool?,
              let toggleRepeatContext = json["toggling_repeat_context"] as? Bool?,
              let toggleShuffle = json["toggling_shuffle"] as? Bool?,
              let toggleRepeatTrack = json["toggling_repeat_track"] as? Bool?,
              let transferPlayback = json["transfer_playback"] as? Bool?
        else {return nil;}
        
        self.interruptPlayback = interruptPlayback;
        self.pause = pause;
        self.resume = resume;
        self.seek = seek;
        self.skipNext = skipNext;
        self.skipPrevious = skipPrevious;
        self.toggleRepeatContext = toggleRepeatContext;
        self.toggleShuffle = toggleShuffle;
        self.toggleRepeatTrack = toggleRepeatTrack;
        self.transferPlayback = transferPlayback;
    }
}
