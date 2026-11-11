# TuviLM

TuviLM makes a **Lá Số Tử Vi** (Vietnamese astrology chart) from a birth date
and hour. Users can then ask an AI agent questions about the chart. The agent
answers using the chart data and the book *Tử Vi Tân Biên*.

## Parts

| Folder | What it is | Tech |
| --- | --- | --- |
| `src/refactored/` | Chart engine: places stars on the chart | Python |
| `src/agent/` | AI agent and its tools | pydantic-ai |
| `api/` | Backend server | FastAPI, MongoDB |
| `astrology_styled_frontend/` | Web app | Next.js, React |
| `frontend/` | Old web app. Do not add features here. | Next.js |

## How it works

```text
Web app (:3000) ──► API (:8000) ──► chart engine
                         │    └───► AI agent ──► LLM (OpenRouter)
                         └────────► MongoDB (users, chats)
```

1. The API gets a birth date, hour and gender.
2. The chart engine builds the chart: `LaSo.from_prior(...)`.
3. The web app draws the chart.
4. In chat, the agent calls tools to read the chart and the book, then answers.
   The answer streams to the browser. Chats are saved in MongoDB.

## Setup

You need: Python 3.12+, [uv](https://docs.astral.sh/uv/), Node 20+,
[pnpm](https://pnpm.io), Docker, and an [OpenRouter](https://openrouter.ai) API key.

**1. Start MongoDB**

```bash
docker compose up -d
```

**2. Start the backend** (from the repo root)

```bash
cp .env.example .env
export OPENROUTER_API_KEY=sk-or-...   # set it in the shell; .env does not work for this key
uv sync
uv run uvicorn api.main:app --reload --port 8000
```

Check it: `curl http://localhost:8000/api/v1/health`

The book files are not in git. The agent reads them from
`data/tuvitanbien_chunking_compact/part_2/`. Ask a maintainer for this folder.
Without it, charts work but the agent cannot read the book.

**3. Start the web app**

```bash
cd astrology_styled_frontend
cp .env.example .env.local    # set BETTER_AUTH_SECRET: openssl rand -base64 32
pnpm install
pnpm dev                      # http://localhost:3000
```

**Optional:** view a chart without the web app:
`uv run streamlit run streamlit_refactored_test.py`

## Folders

```text
api/                      backend routes (main.py, chat/)
src/refactored/           chart engine
  la_so.py                main class: LaSo
  placement/rules/data/   where each star goes (YAML)
  components/data/        star and palace data (JSON)
src/agent/                AI agent
  main.py                 agent prompt and tool list
  tool/                   tools the agent can call
  workflow/               sub-agents (example: personality/)
astrology_styled_frontend/src/
  app/                    pages
  features/               UI for each page
  lib/api/                calls to the backend
data/                     cách cục rules, câu phú text
docs/                     design docs
support_tool/             editor for cách cục rules
tests/                    backend tests
```

## Tests

```bash
uv run pytest             # backend; needs MongoDB running

cd astrology_styled_frontend
pnpm lint && pnpm typecheck && pnpm test
```

There is no CI. Run the tests before you open a PR.

## How to contribute

**Change where a star goes:** edit the YAML files in
`src/refactored/placement/rules/data/`. Add a test in `tests/test_placement_*.py`.
Read `docs/la_so_design/architecture/placement_rules.md` first.

**Add a cách cục (star pattern):** edit `data/cach_cuc_reviewed.yaml`, or use
the editor: `uv run streamlit run support_tool/cach_cuc/cach_cuc_editor.py`.
The rule format is in `docs/la_so_design/cach_cuc_condition_language.md`.

**Add an agent tool:**
1. Write a function in `src/agent/tool/`. It takes `RunContext[TuviAgentDeps]`.
2. Export it in `src/agent/tool/__init__.py`.
3. Add it to `tools=[...]` in `src/agent/main.py`.

The tool should return chart data. The agent writes the explanation.

**Add a sub-agent:** follow `docs/agent_workflows/adding_a_workflow.md`.

**Add a page:** add `src/app/<name>/page.tsx` and put the UI in
`src/features/<name>/`. Call the backend only through `src/lib/api/`.

**Rules:**
- The chart engine (`src/refactored/`) has no network, database or AI calls.
- Code names use Vietnamese words without accents: `cung`, `sao`, `dia_chi`.
- Run all commands from the repo root.
- Fork the repo, make a branch, and open a PR to `main`.

## Docs to read

1. `docs/la_so_design/domain_context.md`: what a chart is
2. `docs/la_so_design/architecture/`: how the chart engine works
3. `docs/conversation_history/`: how chats are stored
4. `api/README.md`: API routes with examples

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

- `/laso/build` keeps one chart for all users. Two users at once see each other's chart.
- The backend does not check who the user is. It trusts the `X-Anonymous-Owner-Id` header.
- CORS allows all websites.
- The AI model name and book path are fixed in `api/main.py`.
