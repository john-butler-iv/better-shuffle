//
//  ContentList.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/15/26.
//

import SwiftUI

struct ContentList: View {
    let data = ["a", "b", "c"]
    var body: some View {
        ScrollView {
            LazyVGrid (columns: [GridItem(.adaptive(minimum: 170, maximum: 170), spacing: 15)], spacing: 15) {
                ForEach(data, id:\.self) { playlist in
                    PlaylistCard()
                }
                SearchCard()
            }
        }
    }
}

#Preview {
    ContentList()
}
