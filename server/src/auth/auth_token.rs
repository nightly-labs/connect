use super::auth_token_type::AuthTokenType;
use chrono::{Duration, Utc};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use serde::{Deserialize, Serialize};
use std::net::{IpAddr, SocketAddr};

#[derive(Debug, Serialize, Deserialize, Clone)]
pub struct AuthToken {
    pub id: String,
    pub user_id: String,
    pub ip: Option<IpAddr>,
    pub token_type: AuthTokenType,
    pub sub: String,
    pub exp: u64, // Required (validate_exp defaults to true in validation). Expiration time (as UTC timestamp)
}

impl AuthToken {
    pub fn new_access(user_id: &String, ip: Option<SocketAddr>) -> Self {
        AuthToken {
            id: uuid7::uuid7().to_string(),
            user_id: user_id.clone(),
            ip: match ip {
                Some(ip) => Some(ip.ip()),
                None => None,
            },
            token_type: AuthTokenType::Access,
            sub: user_id.clone(),
            exp: (Utc::now() + Duration::minutes(30)).timestamp() as u64, // Token expires in 30 minutes
        }
    }
    pub fn new_refresh(user_id: &String, ip: Option<SocketAddr>) -> Self {
        AuthToken {
            id: uuid7::uuid7().to_string(),
            user_id: user_id.clone(),
            ip: match ip {
                Some(ip) => Some(ip.ip()),
                None => None,
            },
            token_type: AuthTokenType::Refresh,
            sub: user_id.clone(),
            exp: (Utc::now() + Duration::minutes(60 * 7 * 24)).timestamp() as u64, // Token expires in 7 days
        }
    }

    pub fn encode(&self, secret: &str) -> Result<String, jsonwebtoken::errors::Error> {
        EncodingKey::from_rsa_pem(secret.as_bytes())
            .map_err(|e| e.into())
            .and_then(|key| encode(&Header::new(Algorithm::RS256), &self, &key))
    }

    pub fn decode(
        token: &str,
        public_key: &str,
        ip: SocketAddr,
    ) -> Result<Self, jsonwebtoken::errors::Error> {
        let decoding_key = DecodingKey::from_rsa_pem(public_key.as_bytes()).map_err(|e| e)?;

        let token_data =
            decode::<AuthToken>(token, &decoding_key, &Validation::new(Algorithm::RS256))?.claims;

        if let Some(token_ip) = token_data.ip {
            if token_ip == ip.ip() {
                Ok(token_data)
            } else {
                Err(jsonwebtoken::errors::Error::from(
                    jsonwebtoken::errors::ErrorKind::InvalidToken,
                ))
            }
        } else {
            // Token does not have to be ip specific
            Ok(token_data)
        }
    }
}
#[cfg(test)]
mod tests {
    use crate::env::{JWT_PUBLIC_KEY, JWT_SECRET};

    use super::*;

    #[test]
    fn test_auth_token_new() {
        // Test the `new` method to create a new `AuthToken` instance.
        let ip = SocketAddr::from(([123, 233, 3, 21], 8080));
        let auth_token = AuthToken::new_access(&"1".to_string(), Some(ip));
        // Check that the `user_id` and `exp` fields are set correctly.
        assert_eq!(auth_token.user_id, "1".to_string());
        assert_eq!(auth_token.ip.unwrap(), ip.ip());
        assert!(auth_token.exp > 0);
    }

    #[test]
    fn test_auth_token_encode() {
        // Test the `encode` method to generate a JWT from an `AuthToken` instance.
        let ip = SocketAddr::from(([123, 233, 3, 21], 8080));
        let auth_token = AuthToken::new_access(&"1".to_string(), Some(ip));
        let token = auth_token.encode(JWT_SECRET()).unwrap();

        // Check that the JWT is a non-empty string.
        assert!(!token.is_empty());
    }

    #[test]
    fn test_auth_token_decode() {
        // Test the `decode` method to parse a JWT and create an `AuthToken` instance.
        let ip = SocketAddr::from(([123, 233, 3, 21], 8080));
        let auth_token = AuthToken::new_access(&"1".to_string(), Some(ip));
        let token = auth_token.encode(JWT_SECRET()).unwrap();
        let decoded_auth_token = AuthToken::decode(&token, JWT_PUBLIC_KEY(), ip).unwrap();

        // Check that the decoded `AuthToken` instance has the same `user_id` and `exp` fields as the original.
        assert_eq!(decoded_auth_token.ip, auth_token.ip);
        assert_eq!(decoded_auth_token.user_id, auth_token.user_id);
        assert_eq!(decoded_auth_token.exp, auth_token.exp);
    }
    #[test]
    fn test_auth_token_decode_invalid_token() {
        // Test the `decode` method with an invalid JWT.
        let token = "invalid-token";
        let ip = SocketAddr::from(([123, 233, 3, 21], 8080));

        // Call the `decode` method and use a `match` expression to handle the error case explicitly.
        let result = AuthToken::decode(token, JWT_SECRET(), ip);
        match result {
            Ok(_) => panic!("Expected an error but got a successful result"),
            _ => (),
        }
    }

    #[test]
    fn test_auth_token_decode_incorrect_secret() {
        // Test the `decode` method with the incorrect public key.
        let ip = SocketAddr::from(([123, 233, 3, 21], 8080));
        let auth_token = AuthToken::new_access(&"1".to_string(), Some(ip));
        let incorrect_public_key = "-----BEGIN PUBLIC KEY-----
        MIICIjANBgkqhkiG9w0BAQEFAAOCAg8AMIICCgKCAgEAzlUYIpqSUAyLJaf8ZUef
        06YBh5DcmaTrwGcVwC57VtywY7bHXQUtGooULQjiYgnyOPxHDt2W+gQW1axiMxOQ
        ew0MF0kXYQhg0+WA1dcWOsxCDpyLc4z+E7EmdV7MTuvsDR2TLhZ6DFmVbN4ca64W
        vftz//3Ptc/I1/VpfGsHWb0VpmphkbE9vWGHzzJVp/Lvtk7ybcpfxiiNWEi7unr7
        /TIEsqy93lCRfpLKIvO06ITH1GZz7+VPL/8q8lrH5kSwxqo5szq1mEa0pNN6hTpo
        cz00x5oBfL9K/TNLQNgdb5uOOfjQ9pYtZ49UDyMLs5lCsZ6ue+SIbWrJXDEPMIEg
        Lzx9nnvytL9wbiwo5GQdIaDAZXMwVVdKQGyqx+UhaHmrb1u0RVRcFzO/QPWzNCWv
        oxJGfg9dVTxfjaa1HzQ0irPs86zCQUnt0OyTuq0SmUgTCNXLSQaN8MigN++zwT4c
        IRDB0WMgmu8sxtnmA3Zq+2GYT97jmbdVMB34DVbXI4UigD3UieVe4rO95cEssoUT
        AFG0pthv64BRKIIRNch+eiBBjkGpnWCRybzsmqnEqpMf9v7KQU9hmpBJDbi6KA6w
        3fg3KSroJZwosbAwdBuylCfQgq1jZ6UfR/ss9kYGhaWbCB3ARmY9XHFc+OZuFW0p
        ohgfBfs9RDMDVkTa4tzBAGsCAwEAAQ==
        -----END PUBLIC KEY-----
        ";

        let token = auth_token.encode(JWT_SECRET()).unwrap();

        // Call the `decode` method and use a `match` expression to handle the error case explicitly.
        let result = AuthToken::decode(&token, incorrect_public_key, ip);
        match result {
            Ok(_) => panic!("Expected an error but got a successful result"),
            _ => (),
        }
    }
    #[test]
    fn test_auth_token_decode_expired_token() {
        // Test the `decode` method with an expired JWT.
        let user_id = "1".to_string();
        let ip = SocketAddr::from(([123, 233, 3, 21], 8080));
        // Set the expiration time of the `AuthToken` instance to a time in the past.
        let exp = (Utc::now() - Duration::minutes(30)).timestamp() as u64;
        let auth_token = AuthToken {
            id: uuid7::uuid7().to_string(),
            user_id: user_id.clone(),
            exp,
            ip: Some(ip.ip()),
            token_type: AuthTokenType::Access,
            sub: user_id,
        };

        let token = auth_token.encode(JWT_SECRET()).unwrap();

        // Call the `decode` method and assert that it returns an error.
        let result = AuthToken::decode(&token, JWT_PUBLIC_KEY(), ip);
        assert!(result.is_err());
    }
    #[test]
    fn test_decode_different_ip() {
        // Test the `decode` method with an expired JWT.
        let ip = SocketAddr::from(([123, 233, 3, 21], 8080));
        let auth_token = AuthToken::new_access(&"1".to_string(), Some(ip));
        let token = auth_token.encode(JWT_SECRET()).unwrap();
        let different_ip = SocketAddr::from(([2, 133, 3, 21], 8080));

        let result = AuthToken::decode(&token, JWT_PUBLIC_KEY(), ip);
        assert!(result.is_ok());
        let result = AuthToken::decode(&token, JWT_PUBLIC_KEY(), different_ip);
        assert!(result.is_err());
    }
}
