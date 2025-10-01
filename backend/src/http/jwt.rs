use std::time::Duration;

use anyhow::Context;
use chrono::{DateTime, Utc};
use jsonwebtoken::{DecodingKey, EncodingKey, Header, TokenData, Validation, decode, encode};
use serde_with::TimestampSeconds;
use uuid::Uuid;

const TOKEN_TTL: Duration = Duration::from_secs(60 * 60 * 24 * 7);

#[serde_with::serde_as]
#[derive(Debug, Clone, Serialize, Deserialize)]
struct Claims {
    /// Whom token refers to (opaque string).
    sub: Uuid,

    /// When this token was issued (UTC timestamp).
    #[serde_as(as = "TimestampSeconds<i64>")]
    iat: DateTime<Utc>,

    /// When this token expires (UTC timestamp).
    #[serde_as(as = "TimestampSeconds<i64>")]
    exp: DateTime<Utc>,
}

pub fn issue_token(sub: Uuid, key: &EncodingKey) -> anyhow::Result<String> {
    let issued_at = Utc::now();
    let claims = Claims {
        sub,
        iat: issued_at,
        exp: issued_at + TOKEN_TTL,
    };
    let token = encode(&Header::default(), &claims, key)
        .map_err(|e| anyhow!(e))
        .context("failed to issue jwt token")?;
    Ok(token)
}

pub fn verify_token(token: impl AsRef<str>, key: &DecodingKey) -> anyhow::Result<Uuid> {
    let TokenData {
        claims: Claims { sub, .. },
        ..
    } = decode::<Claims>(token.as_ref(), key, &Validation::default())?;
    Ok(sub)
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
        let mut secret_bytes = [0; 32];
        password_hash::rand_core::OsRng.fill_bytes(&mut secret_bytes);
        let secret = BASE64_STANDARD.encode(secret_bytes);
        let encoding_key = EncodingKey::from_base64_secret(&secret).unwrap();
        let decoding_key = DecodingKey::from_base64_secret(&secret).unwrap();

        // whom the token is going to refer to; in reality, we rely on the database
        // engine when assigning identifiers to users, here we are generating UUID
        // for test and demonstration purposes solely
        let user_id = Uuid::new_v4();

        // the resulted string will have the following format:
        //
        // "eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9.eyJzdWIiOiIzMGZlOGZkYy03ZjliLTQ5MTktYjBkMC03NmNjNDRjMmZlNTciLCJpYXQiOjE3NTEwNTQ1OTYsImV4cCI6MTc1MTY1OTM5Nn0.b_beenZM34BJt_5xfK5zo7JTy6QPWtIab8WxAsU7Qx8"
        //
        // where:
        //  eyJ0eXAiOiJKV1QiLCJhbGciOiJIUzI1NiJ9        - headers
        //  eyJzdWIiOmV4c...Tc1MTY1OTM5Nn0              - claims
        //  b_beenZM34BJt_5xfK5zo7JTy6QPWtIab8WxAsU7Qx8 - signature
        //
        let token = issue_token(user_id, &encoding_key).unwrap();
        let mut parts = token.split(".");

        let headers = parts.next().unwrap();
        let decoded_header = BASE64_URL_SAFE_NO_PAD
            .decode(headers.as_bytes())
            .map(|bytes| String::from_utf8(bytes).unwrap())
            .unwrap();
        assert_eq!(decoded_header, r#"{"typ":"JWT","alg":"HS256"}"#);

        let claims = parts.next().unwrap();
        let decoded_claims = BASE64_URL_SAFE_NO_PAD
            .decode(claims.as_bytes())
            .map(|bytes| String::from_utf8(bytes).unwrap())
            .unwrap();
        // example of stringified unencoded claims:
        // "{"sub":"25f75337-a5e3-44b1-97d7-6653ca23e9ee","iat":1751116029,"exp":1751720829}"
        assert!(decoded_claims.contains(&format!(r#""sub":"{}""#, user_id)));

        let _signature = parts.next().unwrap();
        assert!(parts.next().is_none());

        assert_eq!(verify_token(token, &decoding_key).unwrap(), user_id);
    }
}
