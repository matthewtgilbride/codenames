# CLAUDE.md

## Project Overview

Codenames is a web-based implementation of the board game [Codenames](https://codenames.game/). It consists of a Rust REST backend and a React/Next.js frontend, deployed to AWS.

## Architecture

```
codenames/
├── service/          # Rust backend
│   ├── domain/       # Core game logic library (pure Rust, no framework deps)
│   └── actix/        # actix-web HTTP server wrapping the domain
├── app/              # Next.js 10 / React 17 frontend (TypeScript, Emotion CSS)
├── infra/            # AWS CDK infrastructure (TypeScript)
└── docker-compose.yml
```

### Backend (Rust)

- **Domain crate** (`service/domain`): Pure game logic — board generation, turns, team management, win conditions. Uses a dictionary file (`default.txt`) for word lists.
- **Actix crate** (`service/actix`): HTTP API layer. Uses Redis for local dev and DynamoDB for production. CORS configured via `ALLOWED_ORIGINS` env var.
- Rust edition: 2018
- Formatting requires nightly: `cargo +nightly fmt`

### Frontend (TypeScript/React)

- Next.js 10 with static export (`next build && next export`)
- Styling: Emotion (`@emotion/css`, `@emotion/styled`)
- Storybook 6 for component development
- ESLint with Airbnb config + Prettier
- No test runner (no unit/integration tests)
- Uses `yarn` as package manager

### Infrastructure

- AWS CDK (TypeScript) in `infra/`
- Static frontend hosted on S3
- Backend runs on EC2 via Docker
- DynamoDB for game persistence in production

## Common Commands

All commands can be run from the repo root via `make`. Run `make help` for the full list.

### Backend

```sh
make test-service          # Run Rust unit tests (cargo test --all)
make format-service        # Format Rust code (requires cargo +nightly)
make check-service         # Format + build + test
make start-service         # Run locally on port 8080
```

### Frontend

```sh
cd app && yarn install     # Install dependencies
make build-app             # Build (next build + next export)
make format-app            # Lint with ESLint --fix
make check-app             # Lint + compile + build
make start-app             # Start dev server (next dev)
cd app && yarn storybook   # Start Storybook on port 6006
```

### Full Stack (Docker)

```sh
make start                 # Start both services via docker-compose
```

## Environment Variables

- `SERVICE_PORT` — Backend port (default: 8080)
- `APP_PORT` — Frontend port (default: 3000)
- `ALLOWED_ORIGINS` — CORS origins for the backend (default: http://localhost:3000)
- `API_URL` — Backend URL the frontend connects to

## Key Conventions

- The root `Makefile` delegates to per-subproject Makefiles in `service/` and `app/`
- Rust formatting uses nightly toolchain (`cargo +nightly fmt`)
- Frontend uses `yarn`, not `npm`
- Integration tests use Postman/Newman (see `service/package.json`) and Python pytest (`service/integration_tests.py`)
- No CI/CD pipeline currently configured
- No frontend unit tests currently exist
