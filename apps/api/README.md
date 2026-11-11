# TuviLM API

FastAPI backend for TuviLM UI.

## Current status
- Real route: `POST /api/v1/laso/build`
- Real route: `GET /api/v1/laso/cau-phu`
- Real route: `POST /api/v1/laso/build_sao_luu`
- Login routes are mounted from `apps/api/src/tuvi_api/auth/routes.py`:
  - `GET /api/v1/auth/me`
  - `POST /api/v1/auth/register`, `POST /api/v1/auth/login`, `POST /api/v1/auth/logout`
- Conversation history routes are mounted from `apps/api/src/tuvi_api/chat/routes.py`:
  - `POST /api/v1/chart-profiles`
  - `GET /api/v1/chart-profiles`
  - `POST /api/v1/chart-profiles/{chart_profile_id}/sessions`
  - `GET /api/v1/chart-profiles/{chart_profile_id}/sessions`
  - `GET /api/v1/sessions/{session_id}`
  - `POST /api/v1/sessions/{session_id}/chat/stream`

## Install dependencies
From repo root:

```bash
uv sync
```

## Who owns the data

Chart profiles and chats belong to an owner id, read from cookies by
`tuvi_api.auth.dependencies.get_owner_id`:

- A logged-in user (cookie `tuvi_session`) owns data as `user_<user id>`.
- A guest gets `anon_<random>` in a signed httpOnly cookie (`tuvi_owner`).
  When the guest registers or logs in, their charts and chats move to the account.

Cookies are `SameSite=Lax` and `Secure` when `AUTH__PUBLIC_URL` is https. The web
app proxies `/api/*` to this server, so the cookies belong to the web app's domain.

## Configuration

Settings come from env vars or `.env` (see `.env.example` at the repo root):

| Variable | Default | Notes |
| --- | --- | --- |
| `CONVERSATION_HISTORY_STORE__URI` | `mongodb://localhost:27017` | MongoDB connection string |
| `CONVERSATION_HISTORY_STORE__DATABASE_NAME` | `tuvilm` | |
| `AUTH__SECRET_KEY` | dev value | Required when `AUTH__PUBLIC_URL` is https |
| `AUTH__PUBLIC_URL` | `http://localhost:3000` | The web app URL as the browser sees it |
| `CORS_ORIGINS` | `["http://localhost:3000"]` | JSON list. Only for browsers that call the API directly |
| `AGENT_MODEL` | `openrouter:qwen/qwen3.7-max` | Needs `OPENROUTER_API_KEY` |
| `BOOK_ROOT` | `data/tuvitanbien_chunking_compact/part_2` | Book files, not in git |

## Run server
From repo root:

```bash
uv run uvicorn tuvi_api.main:app --reload --port 8000
```

## Health check

```bash
curl http://localhost:8000/api/v1/health
```

## Build lá số example

```bash
curl -X POST http://localhost:8000/api/v1/laso/build \
  -H "Content-Type: application/json" \
  -d '{
    "day": 4,
    "month": 4,
    "year": 1998,
    "hour": 8,
    "gender": "M"
  }'
```

## Get the introductory Câu Phú

Call this route after `POST /api/v1/laso/build`. It matches the completed
chart by cung Mệnh position, main stars, Tuần and Triệt, then returns the
original phú text from
`packages/tuvi-engine/src/tuvi_engine/data/cung_menh_phu_luc_bat_v3_tieu_de_moi.json`.

```bash
curl http://localhost:8000/api/v1/laso/cau-phu
```

The response includes `tieu_de`, the unmodified four-line `cau_phu`, and the
same lines separately in `cac_cau`. Calling it before building a chart returns
HTTP `409`.

## Build sao lưu from server state

`/api/v1/laso/build_sao_luu` takes:
- `observation_time`: same shape as build time (`day/month/year/hour/gender`)

```bash
curl -X POST http://localhost:8000/api/v1/laso/build_sao_luu \
  -H "Content-Type: application/json" \
  -d '{
    "observation_time": {
      "day": 8,
      "month": 3,
      "year": 2026,
      "hour": 10,
      "gender": "M"
    }
  }'
```
