//
//  Artist.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/27/26.
//

import Foundation

struct SimplifiedArtist {
    public let id: ArtistID;
    public let name: String;
    
    public let externalURLs: ExternalURLs;
    public let fullDetailsURL: URL;
    
    init?(json: Dictionary<String, AnyObject>) {
        guard let externalURLsJson = json["external_urls"] as? Dictionary<String, AnyObject>,
              let externalURLs = ExternalURLs(json: externalURLsJson),
              let href = json["href"] as? String,
              let url = URL(string: href),
              let id = json["id"] as? String,
              let name = json["name"] as? String,
              let rawType = json["type"] as? String,
              let uri = json["uri"] as? String
        else { return nil; }
        
        if rawType != "artist" { return nil; }
        self.id = ArtistID(id);
        if self.id.uri != SpotifyURI(uri) { return nil; }
        self.name = name;
        self.externalURLs = externalURLs;
        self.fullDetailsURL = url;
    }
    
    var uri: SpotifyURI {
        id.uri
    }
}
