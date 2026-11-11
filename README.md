# TuviLM

TuviLM makes a **Lá Số Tử Vi** (Vietnamese astrology chart) from a birth date
and hour.

TuviLM is one Rust binary with a Postgres database. It replaces the old
Next.js app, FastAPI server and MongoDB. The old code is in git history.

## Parts

| Folder | What it is | Tech |
| --- | --- | --- |
| `crates/tuvi-core/` | Chart engine. No IO, no network, no database. | Rust |
| `crates/tuvi-web/` | Web app: pages, forms, database | [Topcoat](https://github.com/tokio-rs/topcoat), [Toasty](https://github.com/tokio-rs/toasty), Postgres |
| `crates/tuvi-llm/` | LLM client: OpenCode Go and OpenRouter | Rust, reqwest |
| `crates/tuvi-cli/` | Dev commands: `cargo up`, `cargo down` | Rust, [clap](https://docs.rs/clap) |
| `data/` | Cách cục rules and câu phú text, for later features | YAML, JSON |
| `docs/` | Domain notes and the cách cục rule language | Markdown |

## How it works

```text
Browser ──► tuvi-web (:3000, server-rendered HTML) ──► tuvi-core (chart engine)
                    └─────────────────────────────────► Postgres (saved charts)
```

1. On the An sao screen, the user enters a name, a solar birth date (three
   wheels), an hour (a 12-hour clock with AM/PM) and a gender. A preview shows
   the chính tinh of cung Mệnh while the user picks.
2. The form posts to `/charts`. `tuvi-web` checks the input and calls
   `tuvi_core::build_chart`.
3. `tuvi-web` saves the birth data in Postgres and opens `/charts/{id}`.
4. The chart page builds the chart again and draws the 12 cung.

The database stores only the birth data, not the chart. One chart takes about
300 ns to build, so the app builds it on each read. When the engine changes,
you do not need to migrate data.

Topcoat renders all HTML on the server. There is no separate API and no
JavaScript build step. One small static script (`an_sao.js`) adds the wheel
scroll, the clock drag and the confirm dialog to the An sao screen.

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

### LLM key

The AI features need a key from **OpenCode Go** or **OpenRouter**:

```bash
cp .env.example .env          # then set OPENCODE_API_KEY or OPENROUTER_API_KEY
cargo tuvi ask "Tử Vi là gì?" # checks the key: prints the answer as it streams
```

- When both keys are set, OpenCode Go wins. Set `TUVI_LLM_PROVIDER` to choose.
- Set `TUVI_LLM_MODEL` to change the model. On OpenCode Go, use a model with
  the `chat-completions` format: `curl -s https://opencode.ai/zen/go/v1/models`.
  Models with the `messages`, `responses` or Google formats do not work yet.
- `.env.example` lists every setting. `cargo up` and `cargo tuvi` both read `.env`.
- Without a key the app still runs. The chat page says what is missing.

### Accounts

Open the app and press **Tạo tài khoản** (`/register`). Every chart belongs to
the account that saved it. Other accounts get "404 Not Found" for it.

- `/register`, `/login`, `/ho-so` (profile and log out).
- Passwords are hashed with Argon2id. The session token is in a `__Host-`
  cookie (Secure, HttpOnly, SameSite=Lax). The database keeps only its SHA-256
  hash (`user_sessions`). A session lasts 30 days.
- The Secure cookie needs HTTPS. Browsers allow it on `localhost` and
  `127.0.0.1`, so local development works without HTTPS.

Charts saved before accounts existed have no owner, and nobody can open them.
To give them to your account (find your id in the `users` table):

```bash
sudo docker compose exec postgres psql -U tuvi -c "UPDATE saved_charts SET user_id = 1 WHERE user_id IS NULL"
```

### AI chat

Open a chart and press **Hỏi Thiên Hạc**, or go to `/charts/{id}/hoi-ai`.

1. The question is saved in Postgres (`chat_messages`), then the page reloads.
2. The page sends the model a system prompt with the whole chart as text,
   plus the last 20 messages.
3. The answer streams into the page (Topcoat `live!`) and is saved when it ends.
4. If the answer fails, the question stays. A reload tries again.

## Folders

```text
Cargo.toml                        workspace: every crate is in crates/
.cargo/config.toml                aliases: cargo tuvi, cargo up, cargo down
docker-compose.yaml               Postgres
crates/
  tuvi-core/                      chart engine
    src/lib.rs                    module list and public API
    src/chart.rs                  BirthInput, Chart, Cung, build_chart
    src/calendar.rs               solar to lunar date, birth hour branch
    src/calendar/lunar_table.rs   lunar years 1800–2199 (generated, do not edit)
    src/can_chi.rs                Thiên Can and Địa Chi
    src/cung.rs                   12 cung roles, Mệnh, Thân, Thiên Can of each cung
    src/cuc.rs                    Cục
    src/chinh_tinh.rs             the 14 main stars
    src/phu_tinh.rs               the other natal stars, Tràng Sinh ring included
    src/tu_hoa.rs                 Tứ Hóa of the birth year
    src/tuan_triet.rs             Tuần and Triệt
    src/ban_menh.rs               Bản mệnh of the birth year
    src/van_han.rs                Đại hạn and Tiểu hạn of each cung
    scripts/gen_fixtures.py       makes the reference data from the old Python engine
    tests/parity.rs               compares 359 charts with saved reference data
    tests/bench.rs                speed test
  tuvi-web/                       web app
    src/main.rs                   start-up, database connection
    src/app.rs                    root layout, home page (URL /), web manifest
    src/app/charts.rs             POST /charts (saves the An sao form)
    src/app/charts/chart_id.rs    GET /charts/{id}
    src/features/charts/          form checks, database queries, chart grid, accent
    src/features/an_sao/          An sao screen, preview shard, constellations, an_sao.js
    src/features/chat/            chat messages, system prompt, markdown, LLM service
    src/features/auth/            users, sessions, Argon2 passwords, form checks
    src/features/shell/           app navigation: tab bar, rail, brand
    src/app/login.rs, register.rs, logout.rs, ho_so.rs   account pages
    src/app/charts/chart_id/hoi_ai.rs   GET and POST /charts/{id}/hoi-ai
    styles.css                    theme: tokens and utilities (see DESIGN.md)
    assets/                       app icons, crane video, god-ghosts/ portraits (asset!)
    toasty/                       database migrations
  tuvi-llm/                       LLM client (OpenAI chat-completions format)
    src/config.rs                 provider, key, model and URL from the environment
    src/client.rs                 chat: whole answer or a stream of text
    src/sse.rs                    server-sent events parser
    tests/fake_server.rs          end-to-end tests against a local fake server
  tuvi-cli/                       dev commands (cargo tuvi --help)
data/                             data for later features (see data/README.md)
docs/                             domain notes and the design prototype
DESIGN.md                         colors, fonts and layout rules
```

`src/app/` holds pages only: the file path is the URL path. Pages call
`src/features/` for data, rules and the UI parts of each feature.

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

**Change a database model:** edit `crates/tuvi-web/src/features/`. Then, from
`crates/tuvi-web/`:

```bash
cargo run -- toasty migration generate
cargo run -- toasty migration apply
```

Commit the new files in `crates/tuvi-web/toasty/`. Run these commands from
`crates/tuvi-web/`, because Toasty writes `toasty/` in the current folder.

**Add a page:** add a file in `crates/tuvi-web/src/app/`. The file path is the
URL path. Put data and rules in `crates/tuvi-web/src/features/`, not in the page.

**Change the look:** read [`DESIGN.md`](DESIGN.md) first. Use its token classes
(`text-muted`, `glass`, `pill-primary`, ...), not color values.

**Add a premade UI component:** from `crates/tuvi-web/`, run
`topcoat ui add <name>`. It goes in `src/components/`. The Topcoat UI token
names in `styles.css` already point to the design tokens.

**Run without `topcoat dev`:** run `topcoat asset bundle` first. The binary
reads its CSS and JS from `target/<profile>/assets/`. Without them, it does not start.

**Rules:**

- `tuvi-core` has no IO, network, database or AI calls.
- Domain names use Vietnamese words without accents: `cung`, `sao`, `dia_chi`.
- Folder names match crate names: `crates/<crate-name>/`.
- No `unwrap` outside tests. No `unsafe`.
- Make a branch and open a PR to `main`.

## Not done yet

- The Bản mệnh and Vận hạn screens: their tabs are locked ("Sắp có")
- A desktop top bar with the screen title
- Stars that move per period (lưu tinh, Tứ Hóa of a Đại hạn or a year),
  cách cục, câu phú
- AI chat: tools, cách cục and the book Tử Vi Tân Biên (the chat sees only the chart)
- Accounts: Google login, password reset, email check, rate limit on login

## Known issues

- The engine takes the birth hour as Vietnam time. It does not handle time zones.
- A date in a leap month gets the same month number as the regular month.
  The old engine did the same. Some schools of Tử Vi use the next month.
- Supported dates are 1800-01-25 (Tết 1800) to 2199-12-31 22:59.
- The engine copies the old Python rules, also where they differ from common
  Tử Vi tables: Cô Thần for Hợi Tý Sửu years is in Mùi (not Dần), Quả Tú for
  Thân Dậu Tuất years is in Dần (not Mùi), Lưu Hà for Tân years is in Thìn
  (not Mão), and Thiên Quan and Thiên Phúc for Ất and Bính years differ.
- Tiểu hạn goes in the same direction as Đại hạn (by gender and the Âm/Dương
  of the year), as in the old engine. Many schools use gender only: clockwise
  for a man, counter-clockwise for a woman.
- A cung lists its Tứ Hóa in the old engine's id order (Khoa, Kỵ, Lộc, Quyền),
  not in the usual order (Lộc, Quyền, Khoa, Kỵ).
- The An sao preview shard (`POST /an-sao/menh`) needs no login. It only
  builds a chart from the posted numbers, and it saves nothing.
- Without JavaScript, the An sao wheels do not center the picked value in
  every browser, and the clock center does not show the picked hour. The
  form still posts, and the server checks every field.
- Topcoat runtime expressions cannot parse a string to a number, read the
  scroll position or a pointer angle, or use `localStorage`. `an_sao.js` does
  these parts (see `src/features/an_sao.rs`).
- The ghost portraits (`assets/god-ghosts/*.svg`) are 77–414 KB each. The
  page loads one only when its star is in the preview, and the router sends
  it compressed (for example 223 KB becomes 75 KB). The star coordinates
  follow these SVG portraits. The old app's smaller `*-ghost.webp` files are
  square crops that its last version did not use, so they are not ported.
- Topcoat and Toasty are early-stage projects. Expect breaking changes when you
  update them.

- `/register` says when an email already has an account. This lets someone
  test which emails are registered. `/login` does not show this, and a wrong
  email takes as long as a wrong password.

## Docs

- [`DESIGN.md`](DESIGN.md): colors, fonts, accent and layout rules
- [`docs/design/prototype.html`](docs/design/prototype.html): the full design prototype
- [`docs/domain.md`](docs/domain.md): what a chart is and how its parts relate
- [`docs/cach_cuc_rules.md`](docs/cach_cuc_rules.md): the rule language of `data/cach_cuc.yaml`
- [`data/README.md`](data/README.md): the data files

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
