//
//  Image.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/27/26.
//

import Foundation

struct Image {
    public let url: URL
    public let height: Int?
    public let width: Int?
    
    init?(json: Dictionary<String, AnyObject>){
        guard let urlString = json["url"] as? String,
              let url = URL(string: urlString) else { return nil }
        
        self.url = url
        self.height = json["height"] as? Int
        self.width = json["width"] as? Int
    }
}
