# web

The TuviLM web app. It is a [Topcoat](https://github.com/tokio-rs/topcoat)
app with a Postgres database through [Toasty](https://github.com/tokio-rs/toasty).

Read the [README in the repo root](../README.md) for setup, tests and how to
contribute.

Start the app with `cargo up`. To run the steps by hand, use this folder:

```bash
cargo run -- toasty migration apply    # make or update the tables
topcoat dev                            # http://127.0.0.1:3000
```
