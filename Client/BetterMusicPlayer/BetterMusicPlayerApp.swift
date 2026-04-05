//
//  BetterMusicPlayerApp.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/15/26.
//

import SwiftUI

@main
struct BetterMusicPlayerApp: App {
    static let DEBUG_MODE = true;
    var body: some Scene {
        WindowGroup {
            ContentView()
        }
    }
    
    @State private var TrackLists: Array<ShufflableTracklist> = []
    
    func initServer(){
        // TODO
    }
}
