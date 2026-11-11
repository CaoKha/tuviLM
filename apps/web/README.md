# Thiên Hạc — Astrology Frontend

Web client for Thiên Hạc built around the "Luminous Twilight" astrology design.

Read [`AGENTS.md`](./AGENTS.md) before changing anything — it holds the token rules,
breakpoint system, runtime theming contract, and backend contract for this project.

## Getting started

```bash
cp .env.example .env.local
pnpm install
pnpm dev
```

The backend runs separately. The easiest way is to start the whole stack from the
repo root with `uv run tuvi up`, or only the API with:

```bash
uv run uvicorn tuvi_api.main:app --reload
```

The browser only calls `/api/*` on this site. `next.config.ts` forwards those
requests to `API_ORIGIN` (default `http://localhost:8000`), so the login and guest
cookies belong to this site. Login lives in the backend (`/api/v1/auth/*`); the
client side is `src/lib/api/auth.ts`.

## Commands

| Command | What it does |
| --- | --- |
| `pnpm dev` | Dev server (Turbopack) |
| `pnpm build` | Production build |
| `pnpm start` | Serve the build |
| `pnpm lint` | ESLint |
| `pnpm typecheck` | `tsc --noEmit` |
| `pnpm format` | Prettier write |
| `pnpm test` | Vitest |

## Design source of truth

`docs/design/tu-vi-app-responsible.html` is a complete working prototype of all eight
screens. When a question about visual behaviour comes up, read its CSS rather than
guessing.
