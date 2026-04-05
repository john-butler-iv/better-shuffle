//
//  Environment.swift
//  BetterMusicPlayer
//
//  Created by John Butler on 3/26/26.
//

import Foundation

struct Environment {
    let clientId: String
    let clientSecret: String
    let redirectPort: Int16
    let oauthRedirectPath: String
    
    func oauthRedirectURI() -> String {
        return "http://127.0.0.1:\(self.redirectPort)/\(self.oauthRedirectPath)"
    }
    
    init(){
        if let path = Bundle.main.path(forResource: "env", ofType: "json") {
            do {
                  let data = try Data(contentsOf: URL(fileURLWithPath: path), options: .mappedIfSafe)
                  let jsonResult = try JSONSerialization.jsonObject(with: data, options: .mutableLeaves)
                  if let jsonResult = jsonResult as? Dictionary<String, AnyObject>, let clientId = jsonResult["client_id"] as? String, let clientSecret = jsonResult["client_secret"] as? String {
                      
                      self.clientId = clientId;
                      self.clientSecret = clientSecret;
                      self.redirectPort = 3000;
                      self.oauthRedirectPath = "auth_callback";
                  } else { exit(1); }
              } catch { exit(1); }
        } else { exit(1); }
    }
}
