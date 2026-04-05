//
//  PlayerBar.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/15/26.
//

import SwiftUI

struct PlayerBar: View {
    @State private var progress: Double = 0.0
    @State private var total_seconds: Double = 60.0 * 3.23
    
    func formatDuration(numSeconds: Double) -> String {
        let minutes = Int(numSeconds) / 60
        let seconds = Int(numSeconds) % 60;
        return String(format: "%d:%02d", minutes, seconds)
    }
    
    var body: some View {
        VStack{
            VStack(spacing: 0){
                Slider(value: $progress, in: 0.0...total_seconds)
                    .sliderThumbVisibility(.hidden)
                    .padding(0).onSubmit {
                        // TODO set playback time
                    }
                // TODO make the slider go all the way to the edge
                HStack{
                    Text(formatDuration(numSeconds: progress))
                    Spacer()
                    Text(formatDuration(numSeconds: total_seconds))
                }.padding(.horizontal, 5.0).offset(y:-5)
            }
            HStack{
                SwiftUI::Image(uiImage: UIImage(named: "Image")!)
                    .resizable()
                    .aspectRatio(contentMode: .fill)
                    .frame(width: 50, height: 50)
                    .cornerRadius(5)
                VStack(alignment: .leading){
                    Text("Song title")
                    Text("Artist").foregroundStyle(.secondary)
                }
                // TODO make this trail off (as ellipses) and scroll
                Spacer()
                Button(action: {
                    // TODO play/pause
                }) {
                    SwiftUI::Image(systemName: "play.fill")
                }
                .padding(.horizontal, 5.0)
                Button(action: {
                   // TODO fastforward
                }) {
                    SwiftUI::Image(systemName: "forward.fill")
                }
                .padding(.horizontal, 5.0)
            }.padding(.horizontal, 10.0)
        }
    }
}

#Preview {
    VStack{
        Spacer()
        PlayerBar()
    }
}
