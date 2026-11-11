//! The users and user_sessions tables.

/// One account. `email` is stored trimmed and in lower case.
#[derive(Debug, toasty::Model)]
pub struct User {
    #[key]
    #[auto]
    pub id: u64,

    #[unique]
    pub email: String,

    pub display_name: String,

    /// Argon2id, in the PHC string format.
    pub password_hash: String,
}

/// One logged-in browser. The raw token stays in the browser cookie.
#[derive(Debug, toasty::Model)]
pub struct UserSession {
    /// SHA-256 of the session token, as 64 hex characters.
    #[key]
    pub token_hash: String,

    #[index]
    pub user_id: u64,

    /// Unix time in seconds. The session is invalid from this time on.
    pub expires_at: i64,
}
