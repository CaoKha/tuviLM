//! The database connection.

use toasty::Db;
use topcoat::context::{Cx, context};

/// Used when `DATABASE_URL` is not set. Matches `docker-compose.yaml`.
const DEFAULT_DATABASE_URL: &str = "postgres://tuvi:tuvi@localhost:5432/tuvi";

/// Opens the Postgres connection pool with every model in the application.
///
/// The tables come from migrations. After changing a model, generate a migration with
/// `cargo run -- toasty migration generate` and apply it with
/// `cargo run -- toasty migration apply`.
pub async fn connect() -> toasty::Result<Db> {
    let url = std::env::var("DATABASE_URL");
    let url = url.as_deref().unwrap_or(DEFAULT_DATABASE_URL);
    Db::builder()
        .models(toasty::models!(crate::*))
        .connect(url)
        .await
}

/// Clones the database handle stored in the context. The clone uses the same
/// connection pool, so it is cheap to create.
pub fn db(cx: &Cx) -> Db {
    context::<Db>(cx).clone()
}
