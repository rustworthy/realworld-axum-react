use anyhow::Context;
use argon2::Argon2;
use argon2::PasswordHash;
use argon2::PasswordHasher as _;
use argon2::PasswordVerifier;
use argon2::password_hash::Salt;
use argon2::password_hash::SaltString;
use rand::Rng as _;
use rand::TryRngCore;
use rand::distr::Alphanumeric;
use rand::rngs::OsRng;

#[allow(unused)]
pub fn gen_alphanum_string(length: usize) -> String {
    let mut rng = rand::rng();
    std::iter::repeat_with(|| rng.sample(Alphanumeric) as char)
        .take(length)
        .collect()
}

pub fn gen_numeric_string(length: usize) -> String {
    let mut rng = rand::rng();
    std::iter::repeat_with(|| rng.random_range(0..10).to_string())
        .take(length)
        .collect()
}

// with rand@0.8 we generated the salt with `SaltString::generate(&mut OsRng)`,
// while rand@0.9's `OsRng` does not implement the trait expected by the mentioned
// consructor, so we are doing so manually:
pub fn gen_salt_string() -> anyhow::Result<SaltString> {
    let mut bytes = [0u8; Salt::RECOMMENDED_LENGTH];
    let mut rng = OsRng;
    rng.try_fill_bytes(&mut bytes)?;
    let salt =
        SaltString::encode_b64(&bytes).map_err(|_| anyhow!("salt string invariant violated"))?;
    Ok(salt)
}

pub fn hash_password(password: impl AsRef<[u8]>) -> anyhow::Result<String> {
    let salt = gen_salt_string()?;
    // hash password to PHC string ($argon2id$v=19$...)
    let password_hash = Argon2::default()
        .hash_password(password.as_ref(), &salt)
        .map_err(|e| anyhow!(e))
        .context("failed to hash password")?
        .to_string();
    Ok(password_hash)
}

pub fn verify_password(
    password: impl AsRef<[u8]>,
    password_hash: impl AsRef<str>,
) -> anyhow::Result<bool> {
    // parse the PHC string we've created earlier
    let parsed_hash = PasswordHash::new(password_hash.as_ref())
        .map_err(|e| anyhow!(e))
        .context("failed to parse argon2's 'PasswordHash' from string")?;
    // compare with the provided password
    let checks_out = Argon2::default()
        .verify_password(password.as_ref(), &parsed_hash)
        .is_ok();
    Ok(checks_out)
}

#[cfg(test)]
mod tests {

    use super::{hash_password, verify_password};
    use fake::Fake as _;
    use fake::faker::internet::en::Password;

    #[test]
    fn hash_password_then_verify() {
        let password: String = Password(5..10).fake();
        let password_hash = hash_password(&password).unwrap();
        // the resulted string will have the following format:
        //
        // "$argon2id$v=19$m=19456,t=2,p=1$zOROKcCeDIm4ZPUnl2blZA$UZ9RHp7F6uhStHE0yvb2/j9UVfrYShk+1jAyFVxRsX0"
        //
        // where:
        //  argon2id                                    - default algorithm
        //  v=19                                        - default version
        //  m=19456,t=2,p=1                             - default params
        //  zOROKcCeDIm4ZPUnl2blZA                      - salt
        //  UZ9RHp7F6uhStHE0yvb2/j9UVfrYShk+1jAyFVxRsX0 - resulting hash
        assert!(password_hash.contains("argon2id"));
        assert_eq!(password_hash[1..].split('$').count(), 5);
        assert!(verify_password(password, password_hash).unwrap());
    }
}
