//
//  Auth.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/26/26.
//

import Foundation

enum Token
{
    init() {
        self = .PreAuthed
    }
    case PreAuthed
    case Bearer(BearerToken)
    
    static func basicHeader() -> String {
        let env = Environment();
        
        let clientId = env.clientId;
        let clientSecret = env.clientSecret;
        let authString = "\(clientId):\(clientSecret)"
        return "Basic \(Data(authString.utf8).base64EncodedString())"
    }
    
    
    mutating func refresh() async -> BearerToken {
        switch self {
        case .PreAuthed :
            // TODO
            exit(3);
        case .Bearer(var token):
            await token.refresh();
            self = .Bearer(token)
            return token;
        }
    }
    
    mutating func addBearerAuth(urlRequest: inout URLRequest) async {
        let token = await self.refresh();
        urlRequest.setValue("Bearer \(token.accessToken)", forHTTPHeaderField: "Authorization")
    }
}
    
struct BearerToken {
    let accessToken: String
    let expiresAt: Date
    let scopes: Array<Scope>
    let refreshToken: String
    var thing: Bool
    
    init(accessToken: String, expiresAt: Date, scopes: Array<Scope>, refreshToken: String) {
        self.accessToken = accessToken
        self.expiresAt = expiresAt
        self.scopes = scopes
        self.refreshToken = refreshToken
        self.thing = false;
    }
    
    mutating func refresh() async {
        if self.expiresAt < Date.now {
            return
        }
        
        await forceRefresh();
    }
    
    mutating func forceRefresh() async {
        var request = URLRequest(url: URL(string: "https://accounts.spotify.com/api/token")!);
        request.httpMethod = "POST"
        request.setValue("application/x-www-form-urlencoded", forHTTPHeaderField: "Content-Type")
        request.setValue(Token.basicHeader(), forHTTPHeaderField: "Authorization")
        let raw_params = [
            "grant_type": "refresh_token",
            "refresh_token": self.refreshToken,
        ];
        let params = raw_params.map { key, value in
            let encodedKey = key.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? key
            let encodedValue = value.addingPercentEncoding(withAllowedCharacters: .urlQueryAllowed) ?? value
            return "\(encodedKey)=\(encodedValue)"
        }.joined(separator: "&");
        
        request.httpBody = params.data(using: .utf8);
        
        do {
            let (data, _) = try await URLSession.shared.data(for: request)
            let jsonResult = try JSONSerialization.jsonObject(with: data);
            if let jsonResult = jsonResult as? Dictionary<String, AnyObject>{
                self = BearerToken(
                    accessToken: jsonResult["access_token"] as? String ?? "",
                    expiresAt: Date.now + (Double(jsonResult["expires_in"] as? String ?? "0") ?? 0.0),
                    scopes: (jsonResult["scopes"] as? String ?? "").split(separator: " ").map{Scope(String($0))!},
                    refreshToken: jsonResult["refresh_token"] as? String ?? ""
                );
            }
        } catch {
            exit(2)
        }
    }
}


enum Scope {
    case UGCImageUpload
    case UserReadPlaybackState
    case UserModifyPlaybackState
    case UserReadCurrentlyPlaying
    case AppRemoteControl
    case Streaming
    case PlaylistReadPrivate
    case PlaylistReadCollaborative
    case PlaylistModifyPrivate
    case PlaylistModifyPublic
    case UserFollowModify
    case UserFollowRead
    case UserReadPlaybackPosition
    case UserTopRead
    case UserReadRecentlyPlayed
    case UserLibraryModify
    case UserLibraryRead
    case UserReadEmail
    case UserReadPrivate
    case UserPersonalized
    case UserSOALink
    case UserSOAUnlink
    case SOAManageEntitlements
    case SOAManagePartner
    case SOACreatePartner
}

extension Scope: CustomStringConvertible, LosslessStringConvertible {
    init?(_ description: String){
        switch description {
        case "ugc-image-upload": self = .UGCImageUpload;
        case "user-read-playback-state": self = .UserReadPlaybackState;
        case "user-modify-playback-state": self = .UserModifyPlaybackState;
        case "user-read-currently-playing": self = .UserReadCurrentlyPlaying;
        case "app-remote-control": self = .AppRemoteControl;
        case "streaming": self = .Streaming;
        case "playlist-read-private": self = .PlaylistReadPrivate;
        case "playlist-read-collaborative": self = .PlaylistReadCollaborative;
        case "playlist-modify-private": self = .PlaylistModifyPrivate;
        case "playlist-modify-public": self = .PlaylistModifyPublic;
        case "user-follow-modify": self = .UserFollowModify;
        case "user-follow-read": self = .UserFollowRead;
        case "user-read-playback-position": self = .UserReadPlaybackPosition;
        case "user-top-read": self = .UserTopRead;
        case "user-read-recently-played": self = .UserReadRecentlyPlayed;
        case "user-library-modify": self = .UserLibraryModify;
        case "user-library-read": self = .UserLibraryRead;
        case "user-read-email": self = .UserReadEmail;
        case "user-read-private": self = .UserReadPrivate;
        case "user-personalized": self = .UserPersonalized;
        case "user-soa-link": self = .UserSOALink;
        case "user-soa-unlink": self = .UserSOAUnlink;
        case "soa-manage-entitlements": self = .SOAManageEntitlements;
        case "soa-manage-partner": self = .SOAManagePartner;
        case "soa-create-partner": self = .SOACreatePartner;
        default: return nil
        }
    }
    
    var description: String {
        switch self {
        case .UGCImageUpload: return "ugc-image-upload";
        case .UserReadPlaybackState: return "user-read-playback-state";
        case .UserModifyPlaybackState: return "user-modify-playback-state";
        case .UserReadCurrentlyPlaying: return "user-read-currently-playing";
        case .AppRemoteControl: return "app-remote-control";
        case .Streaming: return "streaming";
        case .PlaylistReadPrivate: return "playlist-read-private";
        case .PlaylistReadCollaborative: return "playlist-read-collaborative";
        case .PlaylistModifyPrivate: return "playlist-modify-private";
        case .PlaylistModifyPublic: return "playlist-modify-public";
        case .UserFollowModify: return "user-follow-modify";
        case .UserFollowRead: return "user-follow-read";
        case .UserReadPlaybackPosition: return "user-read-playback-position";
        case .UserTopRead: return "user-top-read";
        case .UserReadRecentlyPlayed: return "user-read-recently-played";
        case .UserLibraryModify: return "user-library-modify";
        case .UserLibraryRead: return "user-library-read";
        case .UserReadEmail: return "user-read-email";
        case .UserReadPrivate: return "user-read-private";
        case .UserPersonalized: return "user-personalized";
        case .UserSOALink: return "user-soa-link";
        case .UserSOAUnlink: return "user-soa-unlink";
        case .SOAManageEntitlements: return "soa-manage-entitlements";
        case .SOAManagePartner: return "soa-manage-partner";
        case .SOACreatePartner: return "soa-create-partner";
        }
    }
}
