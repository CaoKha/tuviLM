mod app;
mod db;
mod features;

use std::process::ExitCode;

#[tokio::main]
async fn main() -> ExitCode {
    // Settings can come from `.env` in this folder or a parent (see `.env.example`).
    let _ = dotenvy::dotenv();

    let db = match db::connect().await {
        Ok(db) => db,
        Err(error) => {
            eprintln!("cannot connect to Postgres (start it with `cargo up`): {error:#}");
            return ExitCode::FAILURE;
        }
    };

    // `cargo run -- toasty migration <command>` manages the database schema.
    let mut args = std::env::args().skip(1).peekable();
    if args.peek().is_some_and(|arg| arg == "toasty") {
        if let Err(error) = toasty_cli::ToastyCli::new(db).parse_from(args).await {
            eprintln!("{error:#}");
            return ExitCode::FAILURE;
        }
        return ExitCode::SUCCESS;
    }

    let chat = features::chat::ChatService::from_env();
    if let Err(reason) = chat.client() {
        eprintln!("AI chat is off: {reason}");
    }

    if let Err(error) = topcoat::start(app::router().with(db).with(chat).build()).await {
        eprintln!("{error:#}");
        return ExitCode::FAILURE;
    }
    ExitCode::SUCCESS
}
