//! Accounts: register, log in, log out, and the current user.
//!
//! - `form`: the register and login forms and their checks
//! - `password`: Argon2 password hashes
//! - `store`: the users and user_sessions tables
//! - `session`: Topcoat session tokens tied to those tables
//!
//! Topcoat keeps the session token in a `__Host-` cookie (Secure, HttpOnly,
//! SameSite=Lax). The database keeps only the token's SHA-256 hash.

mod form;
mod password;
mod session;
mod store;

pub use form::{LoginForm, RegisterForm};
pub use session::{current_user, log_in, log_out, register, require_user};
