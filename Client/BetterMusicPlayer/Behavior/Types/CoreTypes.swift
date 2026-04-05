//
//  CoreTypes.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/26/26.
//

import Foundation

struct SpotifyID: CustomStringConvertible, LosslessStringConvertible, Equatable {
    init(_ description: String) {
        if description.contains(":") {
            let uriPieces = description.split(separator: ":");
            if uriPieces.count == 3 && uriPieces[0] == "spotify" {
                self.rawValue = String(uriPieces[2]);
                return
            }
        }
        self.rawValue = description;
    }
    let rawValue: String
    
    var description: String {
        rawValue
    }
}
struct TrackID: CustomStringConvertible, LosslessStringConvertible, Equatable {
    init(_ description: String) {
        self.rawValue = SpotifyID(description);
    }
    init(rawValue: SpotifyID) {
        self.rawValue = rawValue;
    }
    let rawValue: SpotifyID
    var description: String {
        rawValue.description
    }
    var uri: SpotifyURI {
        return .Track(self)
    }
}
struct PlaylistID: CustomStringConvertible, LosslessStringConvertible, Equatable {
    init(_ description: String) {
        self.rawValue = SpotifyID(description);
    }
    init(rawValue: SpotifyID) {
        self.rawValue = rawValue;
    }
    let rawValue: SpotifyID
    var description: String {
        rawValue.description
    }
    var uri: SpotifyURI {
        return .Playlist(self)
    }
}
struct AlbumID: CustomStringConvertible, LosslessStringConvertible, Equatable  {
    init(_ description: String) {
        self.rawValue = SpotifyID(description);
    }
    init(rawValue: SpotifyID) {
        self.rawValue = rawValue;
    }
    let rawValue: SpotifyID
    var description: String {
        rawValue.description
    }
    var uri: SpotifyURI {
        return .Album(self)
    }
}
struct EpisodeID: CustomStringConvertible, LosslessStringConvertible, Equatable {
    init(_ description: String) {
        self.rawValue = SpotifyID(description);
    }
    init(rawValue: SpotifyID) {
        self.rawValue = rawValue;
    }
    let rawValue: SpotifyID
    var description: String {
        rawValue.description
    }
    var uri: SpotifyURI {
        return .Episode(self)
    }
}
struct ArtistID: CustomStringConvertible, LosslessStringConvertible, Equatable {
    init(_ description: String) {
        self.rawValue = SpotifyID(description);
    }
    init(rawValue: SpotifyID) {
        self.rawValue = rawValue;
    }
    let rawValue: SpotifyID
    var description: String {
        rawValue.description
    }
    var uri: SpotifyURI {
        return .Artist(self)
    }
}
struct ShowID: CustomStringConvertible, LosslessStringConvertible, Equatable {
    init(_ description: String) {
        self.rawValue = SpotifyID(description);
    }
    init(rawValue: SpotifyID) {
        self.rawValue = rawValue;
    }
    let rawValue: SpotifyID
    var description: String {
        rawValue.description
    }
    var uri: SpotifyURI {
        return .Show(self)
    }
}

enum SpotifyURI: CustomStringConvertible, LosslessStringConvertible, Equatable {
    case Track(TrackID)
    case Playlist(PlaylistID)
    case Album(AlbumID)
    case Artist(ArtistID)
    case Episode(EpisodeID)
    case Show(ShowID)
    
    init?(_ description: String) {
        let pieces = description.split(separator: ":")
        
        if pieces.count != 3 { return nil }
        if pieces[0] != "spotify" { return nil }
        let id = String(pieces[2]);
        switch pieces[1] {
        case "track": self = .Track(TrackID(id))
        case "playlist": self = .Playlist(PlaylistID(id))
        case "album": self = .Album(AlbumID(id))
        case "artist": self = .Artist(ArtistID(id))
        case "episode": self = .Episode(EpisodeID(id))
        case "show": self = .Show(ShowID(id))
        default: return nil;
        }
    }
    
    var spotifyID: SpotifyID {
        switch self {
        case .Track(let trackID):
            trackID.rawValue
        case .Playlist(let playlistID):
            playlistID.rawValue
        case .Album(let albumID):
            albumID.rawValue
        case .Artist(let artistID):
            artistID.rawValue
        case .Episode(let episodeID):
            episodeID.rawValue
        case .Show(let showID):
            showID.rawValue
        }
    }
    
    var description: String {
        switch self {
        case .Track(let trackID):
            return "spotify:track:\(trackID)"
        case .Playlist(let playlistID):
            return "spotify:playlist:\(playlistID)"
        case .Album(let albumID):
            return "spotify:album:\(albumID)"
        case .Artist(let artistID):
            return "spotify:artist:\(artistID)"
        case .Episode(let episodeID):
            return "spotify:episode:\(episodeID)"
        case .Show(let showID):
            return "spotify:show:\(showID)"
        }
    }
    
}

enum Restrictions {
    case Market
    case Product
    case Explicit
    case Unknown
    
    init?(json: Dictionary<String, AnyObject>) {
        if let reasonString = json["reason"] as? String {
            self.init(reasonString: reasonString)
        } else {
            return nil;
        }
    }
    init(reasonString: String){
        switch reasonString {
        case "market":
            self = .Market
        case "product":
            self = .Product
        case "explicit":
            self = .Explicit
        default:
            self = .Unknown
        }
    }
}

struct ExternalURLs {
    let spotifyURL: String
    
    init?(json: Dictionary<String, AnyObject>){
        if let spotifyURL = json["spotify"] as? String {
            self.spotifyURL = spotifyURL
        } else {
            return nil;
        }
    }
    init(spotifyURL: String) {
        self.spotifyURL = spotifyURL
    }
}

struct ExternalIDs {
    let isrc: String
    let ean: String
    let upc: String
    
    init?(json: Dictionary<String, AnyObject>){
        if let isrc = json["isrc"] as? String,
           let ean = json["ean"] as? String,
           let upc = json["upc"] as? String {
            self.isrc = isrc;
            self.ean = ean;
            self.upc = upc;
        } else {
            return nil;
        }
    }
}

struct Context {
    let type: ContextType
    let fullDetailsURL: URL
    let externalURLs: ExternalURLs
    
    init?(json: Dictionary<String, AnyObject>) {
        guard let rawType = json["type"] as? String,
              let href = json["href"] as? String,
              let url = URL(string: href),
              let externalURLsJson = json["external_urls"] as? Dictionary<String, AnyObject>,
              let externalURLs = ExternalURLs(json: externalURLsJson),
              let uriString = json["uri"] as? String,
              let uri = SpotifyURI(uriString),
              let type = ContextType(uri: uri)
        else {return nil;}
        
        switch type {
        case .Artist: if rawType != "artist"{ return nil;}
        case .Playlist: if rawType != "playlist"{ return nil;}
        case .Album: if rawType != "album" {return nil;}
        case .Show: if rawType != "show" {return nil;}
        }
        
        self.type = type;
        self.fullDetailsURL = url;
        self.externalURLs = externalURLs;
    }
    
    var uri: SpotifyURI {
        type.uri
    }
}

enum ContextType {
    case Artist(ArtistID)
    case Playlist(PlaylistID)
    case Album(AlbumID)
    case Show(ShowID)
    
    init?(uri: SpotifyURI){
        switch uri {
        case .Artist(let artistID):
            self = .Artist(artistID)
        case .Playlist(let playlistID):
            self = .Playlist(playlistID)
        case .Album(let albumID):
            self = .Album(albumID)
        case .Show(let showID):
            self = .Show(showID)
        default:
            return nil
        }
    }
    
    var uri: SpotifyURI {
        switch self {
        case .Artist(let artistID):
            return artistID.uri
        case .Playlist(let playlistID):
            return playlistID.uri
        case .Album(let albumID):
            return albumID.uri
        case .Show(let showID):
            return showID.uri
        }
    }
}

enum PlayableItem {
    case TrackItem(Track)
    case EpisodeItem(Episode)
    
    init?(json: Dictionary<String, AnyObject>){
        guard let type = json["type"] as? String else { return nil }
        if type == "track" {
            guard let track = Track(json: json) else {return nil;}
            self = .TrackItem(track)
        } else if type == "episode" {
            guard let episode = Episode(json: json) else {return nil;}
            self = .EpisodeItem(episode)
        } else {
            return nil;
        }
    }
    
    var uri: SpotifyURI {
        switch self {
        case .TrackItem(let track): return track.uri
        case .EpisodeItem(let episode): return episode.uri
        }
    }
}
