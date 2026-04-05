//
//  Episode.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/28/26.
//

struct Episode{
    let id: EpisodeID
    init?(json: Dictionary<String, AnyObject>){
        guard let uriString = json["uri"] as? String,
              let uri = SpotifyURI(uriString)
        else {return nil;}
        
        switch uri {
        case .Episode(let episodeID):
            self.id = episodeID;
        default: return nil;
        }
    }
    
    var uri: SpotifyURI {
        .Episode(self.id)
    }
}
