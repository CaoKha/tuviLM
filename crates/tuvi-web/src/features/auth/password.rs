//! Password hashes with Argon2id (default parameters of the `argon2` crate).
//!
//! Hashing is slow on purpose, so it runs on Tokio's blocking threads.

use std::sync::LazyLock;

use argon2::{
    Argon2,
    password_hash::{PasswordHasher, PasswordVerifier, phc::PasswordHash},
};
use topcoat::{Error, Result};

/// A hash that no password matches. Login checks it when the email is unknown,
/// so a wrong email takes as long as a wrong password.
static DUMMY_HASH: LazyLock<String> =
    LazyLock::new(|| hash_blocking("not a real password").unwrap_or_default());

fn hash_blocking(password: &str) -> Result<String> {
    // `hash_password` makes a random salt from the OS.
    Argon2::default()
        .hash_password(password.as_bytes())
        .map(|hash| hash.to_string())
        .map_err(|error| Error::msg(format!("password hash failed: {error}")))
}

fn verify_blocking(password: &str, hash: &str) -> bool {
    PasswordHash::new(hash).is_ok_and(|parsed| {
        Argon2::default()
            .verify_password(password.as_bytes(), &parsed)
            .is_ok()
    })
}

pub async fn hash(password: String) -> Result<String> {
    tokio::task::spawn_blocking(move || hash_blocking(&password))
        .await
        .map_err(|error| Error::msg(format!("password hash task failed: {error}")))?
}

/// Checks `password` against `hash`. With `hash: None` (unknown email), it
/// checks a dummy hash and returns `false`.
pub async fn verify(password: String, hash: Option<String>) -> Result<bool> {
    tokio::task::spawn_blocking(move || match hash {
        Some(hash) => verify_blocking(&password, &hash),
        None => {
            let _ = verify_blocking(&password, &DUMMY_HASH);
            false
        }
    })
    .await
    .map_err(|error| Error::msg(format!("password check task failed: {error}")))
}

#[cfg(test)]
mod tests {
    use super::{hash_blocking, verify_blocking};

    #[test]
    fn hash_then_verify() {
        let Ok(hash) = hash_blocking("mật khẩu 123") else {
            panic!("hash")
        };
        assert!(hash.starts_with("$argon2id$"));
        assert!(verify_blocking("mật khẩu 123", &hash));
        assert!(!verify_blocking("mật khẩu 124", &hash));
        assert!(!verify_blocking("x", "not a hash"));
    }
}
