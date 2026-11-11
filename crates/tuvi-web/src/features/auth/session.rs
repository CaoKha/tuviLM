//! Log in, log out and the current user, on Topcoat sessions.

use std::{
    fmt::Write,
    time::{SystemTime, UNIX_EPOCH},
};

use topcoat::{
    Result,
    context::{Cx, memoize},
    router::{error::RouterErrorExt, href},
    session::{self, Session, TokenHash},
};

use super::{
    LoginForm, RegisterForm,
    form::normalize_email,
    password,
    store::{User, UserSession},
};
use crate::db::db;

/// The logged-in user, as pages need it.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CurrentUser {
    pub id: u64,
    pub email: String,
    pub display_name: String,
}

impl From<User> for CurrentUser {
    fn from(user: User) -> Self {
        Self {
            id: user.id,
            email: user.email,
            display_name: user.display_name,
        }
    }
}

fn hex(hash: &TokenHash) -> String {
    let mut out = String::with_capacity(64);
    for byte in hash.iter() {
        // Writing to a String cannot fail.
        let _ = write!(out, "{byte:02x}");
    }
    out
}

fn unix_seconds(time: SystemTime) -> i64 {
    time.duration_since(UNIX_EPOCH)
        .map_or(0, |d| i64::try_from(d.as_secs()).unwrap_or(i64::MAX))
}

/// The user of this request's session, or `None`. One database lookup per
/// request, however many callers ask.
pub async fn current_user(cx: &Cx) -> Result<Option<CurrentUser>> {
    lookup_current_user(cx).await.clone()
}

#[memoize]
async fn lookup_current_user(cx: &Cx) -> Result<Option<CurrentUser>> {
    let Some(hash) = session::token_hash(cx).await? else {
        return Ok(None);
    };
    let mut db = db(cx);
    let Some(record) = UserSession::filter_by_token_hash(hex(&hash))
        .first()
        .exec(&mut db)
        .await?
    else {
        return Ok(None);
    };
    if record.expires_at <= unix_seconds(SystemTime::now()) {
        record.delete().exec(&mut db).await?;
        return Ok(None);
    }
    let user = User::filter_by_id(record.user_id)
        .first()
        .exec(&mut db)
        .await?;
    Ok(user.map(CurrentUser::from))
}

/// The current user, or a redirect to the login page.
pub async fn require_user(cx: &Cx) -> Result<CurrentUser> {
    let login = href!(crate::app::login::page).resolve(cx);
    Ok(current_user(cx).await?.ok_or_redirect(login)?)
}

async fn start_session(cx: &Cx, user_id: u64) -> Result<()> {
    let Session {
        token_hash,
        expires_at,
    } = session::start(cx).await?;
    toasty::create!(UserSession {
        token_hash: hex(&token_hash),
        user_id: user_id,
        expires_at: unix_seconds(expires_at),
    })
    .exec(&mut db(cx))
    .await?;
    Ok(())
}

/// Creates the account and logs it in. `Ok(Err(message))` is a user mistake.
pub async fn register(cx: &Cx, form: &RegisterForm) -> Result<Result<(), String>> {
    if let Some(problem) = form.problem() {
        return Ok(Err(problem));
    }
    let email = normalize_email(&form.email);
    let mut db = db(cx);
    if User::filter_by_email(&email)
        .first()
        .exec(&mut db)
        .await?
        .is_some()
    {
        return Ok(Err("Email này đã có tài khoản. Hãy đăng nhập.".to_owned()));
    }
    let password_hash = password::hash(form.password.clone()).await?;
    let user = toasty::create!(User {
        email: email,
        display_name: form.display_name.trim(),
        password_hash: password_hash,
    })
    .exec(&mut db)
    .await?;
    start_session(cx, user.id).await?;
    Ok(Ok(()))
}

/// Checks the email and password and logs in. `Ok(false)` means they do not
/// match an account; the message does not say which one was wrong.
pub async fn log_in(cx: &Cx, form: &LoginForm) -> Result<bool> {
    let email = normalize_email(&form.email);
    let user = User::filter_by_email(&email)
        .first()
        .exec(&mut db(cx))
        .await?;
    let hash = user.as_ref().map(|user| user.password_hash.clone());
    if !password::verify(form.password.clone(), hash).await? {
        return Ok(false);
    }
    let Some(user) = user else { return Ok(false) };
    start_session(cx, user.id).await?;
    Ok(true)
}

/// Ends this browser's session and deletes its record.
pub async fn log_out(cx: &Cx) -> Result<()> {
    if let Some(hash) = session::stop(cx).await? {
        UserSession::filter_by_token_hash(hex(&hash))
            .delete()
            .exec(&mut db(cx))
            .await?;
    }
    Ok(())
}
