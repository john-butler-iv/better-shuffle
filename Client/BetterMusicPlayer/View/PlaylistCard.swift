//
//  PlaylistCard.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/15/26.
//

import SwiftUI

struct PlaylistCard: View {
    static let IMAGE_SIZE: CGFloat = 170.0;
    static let BUTTON_SIZE: CGFloat = 35.0;
    static let BUTTON_PADDING: CGFloat = 10.0;
    var body: some View {
        ZStack{
            Button(action: {
                // TODO shuffle playlist or play album
            }){
                SwiftUI::Image(uiImage: UIImage(named: "Image")!)
                    .resizable()
                    .aspectRatio(contentMode: .fill)
                    .frame(width: Self.IMAGE_SIZE, height: Self.IMAGE_SIZE)
                    .cornerRadius(10)
                    .shadow(radius: 10)
            }.onLongPressGesture {
                // TODO Show list -- maybe just context menu?
            }
        }
    }
}

#Preview {
    HStack {
        Spacer()
        PlaylistCard()
        Spacer()
        PlaylistCard()
        Spacer()
    }
}
