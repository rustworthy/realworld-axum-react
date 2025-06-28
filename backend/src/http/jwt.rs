use std::time::Duration;

use anyhow::Context;
use chrono::{DateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use rocket::serde::{Deserialize, Serialize};
use serde_with::TimestampSeconds;

const TOKEN_TTL: Duration = Duration::from_secs(60 * 60 * 24 * 7);

#[serde_with::serde_as]
#[derive(Debug, Serialize, Deserialize)]
#[serde(crate = "rocket::serde")]
pub struct Claims {
    /// Whom token refers to (opaque string).
    sub: String,

    /// When this token was issued (UTC timestamp).
    #[serde_as(as = "TimestampSeconds<i64>")]
    iat: DateTime<Utc>,

    /// When this token expires (UTC timestamp).
    #[serde_as(as = "TimestampSeconds<i64>")]
    exp: DateTime<Utc>,
}

#[allow(unused)]
pub fn issue_token(
    secret: impl AsRef<str>,
    sub: String,
    ttl: Option<Duration>,
) -> anyhow::Result<String> {
    let issued_at = Utc::now();
    let expires_at = issued_at + ttl.unwrap_or(TOKEN_TTL);
    let claims = Claims {
        sub,
        iat: issued_at,
        exp: expires_at,
    };
    let key = EncodingKey::from_base64_secret(secret.as_ref())
        .map_err(|e| anyhow::anyhow!(e))
        .context("failed to create jwt encoding key from provided base64-encoded secret")?;
    let token = encode(&Header::default(), &claims, &key)
        .map_err(|e| anyhow::anyhow!(e))
        .context("failed to issue jwt token")?;
    Ok(token)
}

#[allow(unused)]
pub fn verify_token(secret: impl AsRef<str>, token: impl AsRef<str>) -> anyhow::Result<String> {
    let key = DecodingKey::from_base64_secret(secret.as_ref())
        .map_err(|e| anyhow::anyhow!(e))
        // TODO: this should never happen, sentinels exist to help us avoid such
        // issues at runtime
        .context("failed to create jwt decoding key from provided base64-encoded secret")?;
    let TokenData { claims, .. } = decode::<Claims>(token.as_ref(), &key, &Validation::default())?;
    Ok(claims.sub)
}

#[cfg(test)]
mod tests {
    use super::*;
    use argon2::password_hash;
    use base64::prelude::*;
    use password_hash::rand_core::RngCore as _;
    use uuid::Uuid;

    #[test]
    fn issue_token_then_verify() {
        // 256-bit key for HS256 algorithm, in reality you might want to use
        // `openssl rand -base64 32` to generate a secret key
        let mut key = [0; 32];
        password_hash::rand_core::OsRng.fill_bytes(&mut key);
        let key = BASE64_STANDARD.encode(key);

        // whom the token is going to refer to; in reality, we rely on the database
        // engine when assigning identifiers to users, here we are generating UUID
        // for test and demonstration purposes solely
        let user_id = Uuid::new_v4().to_string();

        // the resulted string will have the following format:
        //
        // "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzMGZlOGZkYy03ZjliLTQ5MTktYjBkMC03NmNjNDRjMmZlNTciLCJpYXQiOjE3NTEwNTQ1OTYsImV4cCI6MTc1MTY1OTM5Nn0.b_beenZM34BJt_5xfK5zo7JTy6QPWtIab8WxAsU7Qx8"
        //
        // where:
        //  eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9        - headers
        //  eyJzdWIiOmV4c...Tc1MTY1OTM5Nn0              - claims
        //  b_beenZM34BJt_5xfK5zo7JTy6QPWtIab8WxAsU7Qx8 - signature
        //
        let token = issue_token(&key, user_id.clone(), None).unwrap();
        let mut parts = token.split(".");

        let headers = parts.next().unwrap();
        let decoded_header = BASE64_URL_SAFE_NO_PAD
            .decode(headers.as_bytes())
            .and_then(|bytes| Ok(String::from_utf8(bytes).unwrap()))
            .unwrap();
        assert_eq!(decoded_header, r#"{"typ":"JWT","alg":"HS256"}"#);

        let claims = parts.next().unwrap();
        let decoded_claims = BASE64_URL_SAFE_NO_PAD
            .decode(claims.as_bytes())
            .and_then(|bytes| Ok(String::from_utf8(bytes).unwrap()))
            .unwrap();
        // example of stringified unencoded claims:
        // "{"sub":"25f75337-a5e3-44b1-97d7-6653ca23e9ee","iat":1751116029,"exp":1751720829}"
        assert!(decoded_claims.contains(&format!(r#""sub":"{}""#, user_id)));

        let _signature = parts.next().unwrap();
        assert!(parts.next().is_none());

        assert_eq!(verify_token(&key, token).unwrap(), user_id);
    }
}
