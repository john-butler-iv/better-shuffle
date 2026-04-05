//
//  Album.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/27/26.
//

import Foundation

struct Album {
    public let id: AlbumID
    public let name: String
    
    public let type: AlbumType
    public let totalTracks: Int
    public let coverArts: Array<Image>
    public let releaseDate: ReleaseDate
    public let artists: Array<SimplifiedArtist>
    
    
    public let externalURLs: ExternalURLs
    public let fullDetailsURL: URL
    public let restrictions: Restrictions
    
    var uri: SpotifyURI {
        self.id.uri
    }
    
    init?(json: Dictionary<String, AnyObject>) {
        guard let albumTypeString = json["album_type"] as? String,
              let albumType = AlbumType(typeString: albumTypeString),
              let totalTracks = json["total_tracks"] as? Int,
              let externalURLsJson = json["external_urls"] as? Dictionary<String, AnyObject>,
              let externalURLs = ExternalURLs(json: externalURLsJson),
              let href = json["href"] as? String,
              let id = json["id"] as? String,
              let rawImages = json["images"] as? Array<Dictionary<String, AnyObject>>,
              let name = json["name"] as? String,
              let releaseDate = json["release_date"] as? String,
              let releaseDatePrecision = json["release_date_precision"] as? String,
              let restrictionsJson = json["restrictions"] as? Dictionary<String, AnyObject>,
              let restrictions = Restrictions(json: restrictionsJson),
              let rawType = json["type"] as? String,
              let uri = json["uri"] as? String,
              let rawArtists = json["artists"] as? Array<Dictionary<String, AnyObject>>
        else { return nil; }
        
        if rawType != "album" { return nil; }
        
        self.type = albumType;
        self.totalTracks = totalTracks
        self.externalURLs = externalURLs;
        if let url = URL(string: href) {
            self.fullDetailsURL = url;
        } else { return nil; }
        self.id = AlbumID(id)
        if SpotifyURI(uri) != self.id.uri { return nil; }
        var images: Array<Image> = [];
        for rawImage in rawImages {
            if let image = Image(json: rawImage) {
                images.append(image);
            } else { return nil; }
        }
        self.coverArts = images;
        self.name = name;
        if let releaseDate = ReleaseDate(dateString: releaseDate, precision: releaseDatePrecision) {
            self.releaseDate = releaseDate;
        } else { return nil; }
        self.restrictions = restrictions;
        var artists: Array<SimplifiedArtist> = [];
        for rawArtist in rawArtists {
            if let artist = SimplifiedArtist(json: rawArtist) {
                artists.append(artist);
            } else { return nil; }
        }
        self.artists = artists;
    }
}

enum AlbumType {
    case Album
    case Single
    case Compilation
    init?(typeString: String){
        switch(typeString){
        case "album": self = .Album;
        case "single": self = .Single;
        case "compilation": self = .Compilation;
        default: return nil;
        }
    }
}

enum ReleaseDate {
    case Year(Date)
    case Month(Date)
    case Day(Date)
    
    init?(dateString: String, precision: String){
        let pieces = dateString.split(separator: "-");
        
        switch (precision) {
        case "year":
            if pieces.count < 1 {return nil;}
            guard let year = Int(pieces[0]) else { return nil; }
            
            var dateComponents = DateComponents();
            dateComponents.year = year;
            
            guard let date = Calendar.current.date(from: dateComponents) else { return nil };
            
            self = .Year(date);
        case "month":
            if pieces.count < 2 {return nil;}
            guard let year = Int(pieces[0]),
                  let month = Int(pieces[1]) else { return nil; }
            
            var dateComponents = DateComponents();
            dateComponents.year = year;
            dateComponents.month = month;
            
            guard let date = Calendar.current.date(from: dateComponents) else { return nil };
            
            self = .Month(date);
        case "day":
            if pieces.count < 3 {return nil;}
            guard let year = Int(pieces[0]),
                  let month = Int(pieces[1]),
            let day = Int(pieces[2]) else { return nil; }
            
            var dateComponents = DateComponents();
            dateComponents.year = year;
            dateComponents.month = month;
            dateComponents.day = day;

            guard let date = Calendar.current.date(from: dateComponents) else { return nil };
            
            self = .Day(date);
        default: return nil;
        }
    }
    
    var date: Date {
        switch(self){
        case .Year(let date): return date;
        case .Month(let date): return date;
        case .Day(let date): return date;
        }
    }
}
