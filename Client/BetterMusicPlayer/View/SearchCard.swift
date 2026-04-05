//
//  SearchCard.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/15/26.
//

import SwiftUI


struct SearchCard: View {
    static let labelText = "";// "Add a new track list";
    var body: some View {
        ZStack{
            RoundedRectangle(cornerRadius: 10)
                .fill(Color.white)
                .frame(width: 170, height: 170)
                .shadow(radius: 10)
            VStack{
                Text(Self.labelText).hidden()
                SwiftUI::Image(systemName: "plus")
                    .imageScale(.large)
                    .bold()
                Text(Self.labelText)
            }
        }.onTapGesture {
            // TODO
        }
    }
}

#Preview {
    HStack {
        Spacer()
        SearchCard()
        Spacer()
        SearchCard()
        Spacer()
    }
}
