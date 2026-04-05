//
//  SpotifyActions.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/27/26.
//

import Foundation;

func lookupTrack(trackID: TrackID, token: inout Token) async -> Track {
    var request = URLRequest(url: URL(string: "https://api.spotify.com/v1/tracks/\(trackID)")!);
    request.httpMethod = "GET"
    
    await token.addBearerAuth(urlRequest: &request);
    
    do {
        let (data, _) = try await URLSession.shared.data(for: request)
        let jsonResult = try JSONSerialization.jsonObject(with: data);
        return Track(json: (jsonResult as? Dictionary<String, AnyObject>)!)!
    } catch {
        exit(2)
    }
}

func lookupCurrentTrack(token: inout Token) async -> PlaybackState? {
    var request = URLRequest(url: URL(string: "https://api.spotify.com/v1/me/player/currently-playing")!);
    request.httpMethod = "GET";
    
    await token.addBearerAuth(urlRequest: &request);
    
    do {
        let (data, _) = try await URLSession.shared.data(for: request)
        let jsonResult = try JSONSerialization.jsonObject(with: data);
        guard let json = (jsonResult as? Dictionary<String, AnyObject>)
        else { return nil; }
        
        return PlaybackState(json: json);
    } catch { return nil;}
}

func lookupQueue(token: inout Token) async -> [Track]? {
    var request = URLRequest(url: URL(string: "https://api.spotify.com/v1/me/player/queue")!);
    request.httpMethod = "GET";
    
    await token.addBearerAuth(urlRequest: &request);
    
    do {
        let (data, _) = try await URLSession.shared.data(for: request)
        let jsonResult = try JSONSerialization.jsonObject(with: data);
        guard let json = (jsonResult as? Dictionary<String, AnyObject>)
        else { return []; }
        
        var outputQueue: Array<Track> = [];
        
        if let currentSongJson = json["currently_playing"] as? Dictionary<String, AnyObject>,
              let currentSong = Track(json: currentSongJson)
        {
            outputQueue.append(currentSong)
        }
        
        if let queueJson = json["queue"] as? [Dictionary<String, AnyObject>] {
            for trackJson in queueJson {
                if let track = Track(json: trackJson) {
                    outputQueue.append(track)
                }
            }
        }
        
        return outputQueue
        
    } catch {
        exit(2)
    }
}

func addToQueue(uri: SpotifyURI, token: inout Token) async {
    // TODO
}
func beginPlayback(firstSong: SpotifyURI, contextId: SpotifyURI, token: inout Token) async {
    // TODO
}
