//
//  Track.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/27/26.
//

import Foundation;

struct Track {
    public let id: TrackID;
    public let name: String;
    public let duration: Duration;
    
    public let album: Album;
    public let artists: Array<SimplifiedArtist>;
    
    public let discNumber: Int;
    public let trackNumber: Int;
    public let isExplicit: Bool;
    public let isPlayable: Bool;
    public let isLocal: Bool;

    public let externalIDs: ExternalIDs;
    public let externalURLs: ExternalURLs;
    public let fullDetailsURL: URL;
    public let restrictions: Restrictions;
    
    var uri: SpotifyURI {
        return id.uri
    }
    
    init?(json: Dictionary<String, AnyObject>) {
        guard let albumJson = json["album"] as? Dictionary<String, AnyObject>,
              let album = Album(json: albumJson),
              let rawArtists = json["artists"] as? Array<Dictionary<String, AnyObject>>,
              let discNumber = json["disc_number"] as? Int,
              let durationMS = json["duration_ms"] as? Int,
              let explicit = json["is_explicit"] as? Bool,
              let externalIDsJson = json["external_ids"] as? Dictionary<String, AnyObject>,
              let externalIDs = ExternalIDs(json: externalIDsJson),
              let externalURLsJson = json["external_urls"] as? Dictionary<String, AnyObject>,
              let externalURLs = ExternalURLs(json: externalURLsJson),
              let href = json["href"] as? String,
              let url = URL(string: href),
              let id = json["id"] as? String,
              let isPlayable = json["is_playable"] as? Bool,
              let restrictionsJson = json["restrictions"] as? Dictionary<String, AnyObject>,
              let restrictions = Restrictions(json: restrictionsJson),
              let name = json["name"] as? String,
              let trackNumber = json["track_number"] as? Int,
              let rawType = json["type"] as? String,
              let uri = json["uri"] as? String,
              let isLocal = json["is_local"] as? Bool
        else { return nil; }
        
        if rawType != "track" { return nil; }
        
        self.album = album;
        var artists: Array<SimplifiedArtist> = [];
        for rawArtist in rawArtists {
            if let artist = SimplifiedArtist(json: rawArtist) {
                artists.append(artist);
            } else { return nil; }
        }
        self.artists = artists;
        self.discNumber = discNumber;
        self.duration = Duration.milliseconds(durationMS);
        self.isExplicit = explicit;
        self.externalIDs = externalIDs;
        self.externalURLs = externalURLs;
        self.fullDetailsURL = url;
        self.id = TrackID(id);
        if self.id.uri != SpotifyURI(uri) { return nil; }
        self.isPlayable = isPlayable;
        self.restrictions = restrictions;
        self.name = name;
        self.trackNumber = trackNumber;
        self.isLocal = isLocal;
    }
    
}
