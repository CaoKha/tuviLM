# tuvi-web

The TuviLM web app. It is a [Topcoat](https://github.com/tokio-rs/topcoat)
app with a Postgres database through [Toasty](https://github.com/tokio-rs/toasty).

Start it with `cargo up` from any folder in the repo. Read the
[README in the repo root](../../README.md) for setup, folders and how to
contribute.

To run the steps by hand, use this folder:

```bash
cargo run -- toasty migration apply    # make or update the tables
topcoat dev                            # http://127.0.0.1:3000
```
