//
//  ContentView.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/15/26.
//

import SwiftUI

struct ContentView: View {
    var body: some View {
        VStack {
            ContentList()
            Spacer()
            PlayerBar()
        }
    }
}

#Preview {
    ContentView()
}
