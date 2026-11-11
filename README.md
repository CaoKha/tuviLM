# TuviLM

TuviLM makes a **Lá Số Tử Vi** (Vietnamese astrology chart) from a birth date
and hour. Users can then ask an AI agent questions about the chart. The agent
answers using the chart data and the book *Tử Vi Tân Biên*.

## Parts

The repo is a monorepo. The Python packages are one [uv workspace](https://docs.astral.sh/uv/concepts/projects/workspaces/).

| Folder | What it is | Tech |
| --- | --- | --- |
| `packages/tuvi-engine/` | Chart engine: places stars on the chart (`tuvi_engine`) | Python |
| `packages/tuvi-agent/` | AI agent and its tools (`tuvi_agent`) | pydantic-ai |
| `apps/api/` | Backend server: charts, chat, login (`tuvi_api`) | FastAPI, MongoDB |
| `apps/web/` | Web app | Next.js, React |
| `tools/` | `tuvi` CLI and Streamlit tools (`tuvi_tools`) | Python |

## How it works

```text
Browser ──► Web app (:3000) ──/api/*──► API (:8000) ──► chart engine
                                          │    └──────► AI agent ──► LLM (OpenRouter)
                                          └───────────► MongoDB (users, sessions, charts, chats)
```

1. The browser only talks to the web app. The web app forwards every `/api/*`
   request to the API (`apps/web/next.config.ts`).
2. The API gets a birth date, hour and gender. The chart engine builds the
   chart: `LaSo.from_prior(...)`. The web app draws it.
3. In chat, the agent calls tools to read the chart and the book, then answers.
   The answer streams to the browser. Chats are saved in MongoDB.
4. Charts and chats belong to an owner. A guest gets an anonymous owner in a
   signed cookie. When the guest registers or logs in, the data moves to the account.

## Setup

You need: Python 3.12+, [uv](https://docs.astral.sh/uv/), Node 20+,
[pnpm](https://pnpm.io), Docker, and an [OpenRouter](https://openrouter.ai) API key.

```bash
uv run tuvi setup     # install deps, create .env and apps/web/.env.local
# put OPENROUTER_API_KEY=sk-or-... in .env
uv run tuvi up        # MongoDB + API + web app; Ctrl+C stops all
```

Open http://localhost:3000. The API docs are at http://localhost:8000/docs.
More options are in `docs/local_development.md`.

The book files are not in git. The agent reads them from
`data/tuvitanbien_chunking_compact/part_2/`. Ask a maintainer for this folder.
Without it, charts work but the agent cannot read the book.

**Without the CLI:**

```bash
docker compose up -d                                        # MongoDB
export OPENROUTER_API_KEY=sk-or-...                         # only `tuvi up` reads it from .env
uv run uvicorn tuvi_api.main:app --reload --port 8000       # API, from the repo root
cd apps/web && pnpm install && pnpm dev                     # web app
```

## Folders

```text
apps/api/src/tuvi_api/          backend
  main.py                       app, chart routes
  auth/                         login, sessions, guest owner cookie
  chat/                         chat routes and MongoDB store
packages/tuvi-engine/src/tuvi_engine/
  la_so.py                      main class: LaSo
  placement/rules/data/         where each star goes (YAML)
  components/data/              star and palace data (JSON)
  data/                         câu phú text
packages/tuvi-agent/src/tuvi_agent/
  main.py                       agent prompt and tool list
  tool/                         tools the agent can call
  tool/cach_cuc/data/           cách cục rules (YAML)
  workflow/                     sub-agents (example: personality/)
apps/web/src/
  app/                          pages
  features/                     UI for each page
  lib/api/                      calls to the backend, login hooks
tools/src/tuvi_tools/           tuvi CLI, cách cục editor, chart viewer
docs/                           design docs
tests/                          Python tests
```

## Tests

```bash
uv run tuvi check     # Python tests + web typecheck, lint and tests
```

Tests that need MongoDB are skipped when it is not running. Start it with
`uv run tuvi up` or `docker compose up -d` to run them.

There is no CI. Run the checks before you open a PR.

## How to contribute

**Change where a star goes:** edit the YAML files in
`packages/tuvi-engine/src/tuvi_engine/placement/rules/data/`. Add a test in
`tests/test_placement_*.py`. Read `docs/la_so_design/architecture/placement_rules.md` first.

**Add a cách cục (star pattern):** edit
`packages/tuvi-agent/src/tuvi_agent/tool/cach_cuc/data/cach_cuc_reviewed.yaml`,
or use the editor: `uv run tuvi editor`.
The rule format is in `docs/la_so_design/cach_cuc_condition_language.md`.

**Add an agent tool:**
1. Write a function in `packages/tuvi-agent/src/tuvi_agent/tool/`. It takes `RunContext[TuviAgentDeps]`.
2. Export it in `tool/__init__.py`.
3. Add it to `tools=[...]` in `packages/tuvi-agent/src/tuvi_agent/main.py`.

The tool should return chart data. The agent writes the explanation.

**Add a sub-agent:** follow `docs/agent_workflows/adding_a_workflow.md`.

**Add a page:** in `apps/web/`, add `src/app/<name>/page.tsx` and put the UI in
`src/features/<name>/`. Call the backend only through `src/lib/api/`.

**Add a Python dependency:** `uv add --package <tuvi-engine|tuvi-agent|tuvi-api|tuvi-tools> <dep>`.

**Rules:**
- The chart engine (`tuvi_engine`) has no network, database or AI calls.
- Code names use Vietnamese words without accents: `cung`, `sao`, `dia_chi`.
- Run all commands from the repo root.
- Fork the repo, make a branch, and open a PR to `main`.

## Docs to read

1. `docs/la_so_design/domain_context.md`: what a chart is
2. `docs/la_so_design/architecture/`: how the chart engine works
3. `docs/conversation_history/`: how chats are stored
4. `apps/api/README.md`: API routes, login and settings
5. `docs/local_development.md`: the `tuvi` CLI

## Words

| Word | Meaning |
| --- | --- |
| Lá số | The chart |
| Cung | One of 12 boxes on the chart. Each has a role: Mệnh (self), Tài Bạch (money), … |
| Địa Chi | The 12 positions (Tý to Hợi) |
| Thiên Can | The 10 heavenly stems |
| Sao | Star. Chính tinh = 14 main stars. Phụ tinh = minor stars. |
| Cục | Element from the Mệnh box. It decides where the star Tử Vi goes. |
| Bản mệnh | Element from the birth year |
| Tứ Hóa | Four changes: Lộc, Quyền, Khoa, Kỵ |
| Tuần / Triệt | Two marks that make a box weaker |
| Cách cục | A known star pattern with a meaning |
| Đại hạn / Tiểu hạn | 10-year period / 1-year period |
| Câu phú | A classic verse about the chart |
| Đắc / Hãm | Star in a good place / bad place |

## Known issues

- `/laso/build` keeps one chart for all users. Two users at once see each other's
  chart in `/laso/cau-phu` and `/laso/build_sao_luu`. The chat does not have this problem.
- The chat stream has no rate limit, so one user can spend a lot of LLM credit.
