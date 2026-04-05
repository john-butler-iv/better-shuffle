//
//  Device.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/28/26.
//

struct Device {
    let id: String?
    let name: String
    
    let isActive: Bool
    let isPrivateSession: Bool
    let isRestricted: Bool
    let type: DeviceType
    let volume: Int?
    let supportsVolume: Bool
    
    init?(json: Dictionary<String, AnyObject>){
        guard
            let isActive = json["is_active"] as? Bool,
            let isPrivateSession = json["is_private_session"] as? Bool,
            let isRestricted = json["is_restricted"] as? Bool,
            let name = json["name"] as? String,
            let typeString = json["type"] as? String,
            let supportsVolume = json["supports_volume"] as? Bool
        else { return nil; }
        
        self.id = json["id"] as? String;
        self.name = name;
        self.isActive = isActive;
        self.isPrivateSession = isPrivateSession
        self.isRestricted = isRestricted;
        self.type = DeviceType(string: typeString);
        self.volume = json["volume_percentage"] as? Int;
        self.supportsVolume = supportsVolume;
    }
}

enum DeviceType {
    case Computer
    case SmartPhone
    case Speaker
    case Other
    
    init(string: String) {
        switch string {
        case "computer":
            self = .Computer
        case "smartphone":
            self = .SmartPhone
        case "speaker":
            self = .Speaker
        default:
            self = .Other
        }
    }
}
