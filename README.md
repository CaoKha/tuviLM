# TuviLM

TuviLM makes a **Lá Số Tử Vi** (Vietnamese astrology chart) from a birth date
and hour.

TuviLM is one Rust binary with a Postgres database. It replaces the old
Next.js app, FastAPI server and MongoDB. The old code is in git history.

## Parts

| Folder | What it is | Tech |
| --- | --- | --- |
| `crates/tuvi-core/` | Chart engine. No IO, no network, no database. | Rust |
| `web/` | Web app: pages, forms, database | [Topcoat](https://github.com/tokio-rs/topcoat), [Toasty](https://github.com/tokio-rs/toasty), Postgres |
| `data/` | Cách cục rules and câu phú text, for later features | YAML, JSON |
| `docs/` | Design docs. Some describe the old Python code. | Markdown |

## How it works

```text
Browser ──► web (:3000, server-rendered HTML) ──► tuvi-core (chart engine)
                    └────────────────────────────► Postgres (saved charts)
```

1. The user enters a name, a solar birth date, an hour and a gender.
2. `web` checks the input and calls `tuvi_core::build_chart`.
3. `web` saves the birth data in Postgres and opens `/charts/{id}`.
4. The chart page builds the chart again and draws the 12 cung.

The database stores only the birth data, not the chart. One chart takes about
300 ns to build, so the app builds it on each read. When the engine changes,
you do not need to migrate data.

Topcoat renders all HTML on the server. There is no separate API and no
JavaScript build step.

## Setup

You need: Rust (stable, edition 2024), Docker, and the Topcoat CLI:

```bash
cargo install topcoat-cli
```

Start everything with one command (from any folder in the repo):

```bash
cargo up                  # http://127.0.0.1:3000
```

`cargo up` does three steps:

1. It starts Postgres with `docker compose` and waits until it is ready. It
   skips this step when Postgres already runs on port 5432, or when you set
   `DATABASE_URL`.
2. It applies the database migrations.
3. It starts `topcoat dev`. The dev server rebuilds the app and reloads the
   page when you save a file. Press Ctrl-C to stop it.

`cargo down` stops Postgres. The data stays in the Docker volume.

The app uses `postgres://tuvi:tuvi@localhost:5432/tuvi`. Set `DATABASE_URL` to
use a different database.

If your user is not in the `docker` group, `cargo up` and `cargo down` run
Docker with `sudo` and ask for your password.

## Folders

```text
Cargo.toml                    workspace
crates/tuvi-core/
  src/lib.rs                  public API: BirthInput, Chart, build_chart
  src/calendar.rs             solar to lunar date, hour branch
  src/lunar_table.rs          lunar years 1800–2199 (generated)
  src/cung.rs                 12 cung roles, Mệnh, Thân, Thiên Can of each cung
  src/cuc.rs                  Cục
  src/chinh_tinh.rs           the 14 main stars
  tests/parity.rs             compares 359 charts with saved reference data
web/
  src/app.rs                  layout, home page, chart form
  src/app/charts.rs           POST /charts: save a chart
  src/app/charts/chart_id.rs  GET /charts/{id}: the chart page
  src/features/charts.rs      database model and chart rules
  src/components/             Topcoat UI components (edit them freely)
  toasty/                     database migrations
xtask/                        `cargo up` and `cargo down` (see .cargo/config.toml)
docker-compose.yaml           Postgres
```

## Tests

```bash
cargo test --workspace
cargo clippy --workspace --all-targets -- -D warnings
```

Measure the speed of the engine:

```bash
cargo test --release -p tuvi-core --test bench -- --ignored --nocapture
```

There is no CI. Run the tests before you open a PR.

## How to contribute

**Change the chart engine:** edit `crates/tuvi-core/` and run
`cargo test -p tuvi-core`. The old Python engine made the reference charts in
`tests/fixtures/charts.json`. Change this file only when you change a result on
purpose, and say why in the PR.

**Change a database model:** edit `web/src/features/*.rs`. Then, from `web/`:

```bash
cargo run -- toasty migration generate
cargo run -- toasty migration apply
```

Commit the new files in `web/toasty/`. Run these commands from `web/`, because
Toasty writes `toasty/` in the current folder.

**Add a page:** add a file in `web/src/app/`. The file path is the URL path.
Put data and rules in `web/src/features/`, not in the page.

**Add a UI component:** from `web/`, run `topcoat ui add <name>`.

**Run without `topcoat dev`:** run `topcoat asset bundle` first. The binary
reads its CSS and JS from `target/<profile>/assets/`. Without them, it does not start.

**Rules:**

- `tuvi-core` has no IO, network, database or AI calls.
- Code names use Vietnamese words without accents: `cung`, `sao`, `dia_chi`.
- No `unwrap` outside tests. No `unsafe`.
- Make a branch and open a PR to `main`.

## Not done yet

- Phụ tinh, Tứ Hóa, Tuần/Triệt, Đại hạn/Tiểu hạn, cách cục, câu phú
- AI chat about the chart
- Users and login

## Known issues

- The engine takes the birth hour as Vietnam time. It does not handle time zones.
- A date in a leap month gets the same month number as the regular month.
  The old engine did the same. Some schools of Tử Vi use the next month.
- Supported dates are 1800-01-25 (Tết 1800) to 2199-12-31 22:59.
- Topcoat and Toasty are early-stage projects. Expect breaking changes when you
  update them.

## Words

| Word | Meaning |
| --- | --- |
| Lá số | The chart |
| Cung | One of 12 boxes on the chart. Each has a role: Mệnh (self), Tài Bạch (money), … |
| Địa Chi | The 12 positions (Tý to Hợi) |
| Thiên Can | The 10 heavenly stems |
| Sao | Star. Chính tinh = 14 main stars. Phụ tinh = minor stars. |
| Cục | Element from the Mệnh box. It decides where the star Tử Vi goes. |
| Thân | The second "self" cung. It is marked "(Thân)" on the chart. |
