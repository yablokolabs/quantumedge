# QuantumEdge

QuantumEdge is a production-oriented optimization platform for small and mid-sized UK investment banks. It helps treasury teams, portfolio managers, and advisory desks compare classical baselines against quantum-ready optimization workflows for portfolio construction, rebalancing, treasury allocation, and trade basket selection.

## Positioning
QuantumEdge sells business outcomes, not quantum hype.

It is built to answer questions like:
- can we improve allocation quality versus a classical baseline?
- can we reduce turnover or capital friction under realistic constraints?
- can we prepare today on simulators and hybrid solvers before moving to hardware-backed workflows later?

## What users see
QuantumEdge is meant to produce output that is easy to read, not just technically correct.

Every optimization run should surface:
- baseline vs optimized expected return
- risk change
- budget utilization
- turnover impact
- constraint status
- annualized gain estimate
- allocation changes by asset
- plain-English explanation of what changed and why

### Example result summary

```text
Scenario: Boutique Wealth Desk
Solver mode: Hybrid

Baseline portfolio
- Expected return: 10.9%
- Variance proxy: 8.1%
- Turnover: 19%
- Constraint violations: 1

Optimized portfolio
- Expected return: 12.3%
- Variance proxy: 7.1%
- Turnover: 12%
- Constraint violations: 0
- Budget usage: 99.8%

Business impact
- Estimated annualized gain: £185,000
- Risk posture: improved
- Compliance status: constraints satisfied

Allocation changes
- UK_GILT_5Y: 25% -> 30%
- FTSE_BANKS_BASKET: 20% -> 25%
- GBP_LIQUIDITY: 25% -> 20%
- MIDCAP_ALPHA: 30% -> 25%

Plain-English explanation
The optimized allocation improved expected return while reducing the risk proxy and turnover.
This run stayed within the configured budget and liquidity constraints.
The workflow remains simulator-first / hybrid-ready, so the same business logic can later target D-Wave-backed execution through the adapter layer.
```

That output philosophy is intentional: the machine-readable payload is for integrations, but the README and product should already show the human-readable layer that analysts and decision-makers will actually consume.

## Monorepo layout
- `apps/web` , analyst-facing web app
- `services/rust-api` , auth, workspace, jobs, audit, orchestration API
- `services/python-optimizer` , optimization engine with simulator and hybrid adapters
- `packages/shared-schemas` , shared API and scenario schemas
- `examples` , sample datasets and demo scenarios
- `docs` , architecture, onboarding, and API docs

## Requirements
- Python `3.11+`
- Rust stable toolchain
- Node.js `22+`
- npm
- Git

## Quick start

### 1. Clone the repo
```bash
git clone https://github.com/yablokolabs/quantumedge.git
cd quantumedge
```

### 2. Bootstrap dependencies
```bash
./scripts/bootstrap.sh
```

This script will:
- create the Python virtual environment
- install Python dependencies
- run `cargo check` for the Rust API
- install npm packages for the web app
- build the web app once

## Run the project locally

### Terminal 1, start the Python optimizer
```bash
cd services/python-optimizer
source .venv/bin/activate
uvicorn quantumedge_optimizer.main:app --host 0.0.0.0 --port 8099 --reload
```

### Terminal 2, start the Rust API
```bash
cd services/rust-api
OPTIMIZER_URL=http://127.0.0.1:8099 cargo run
```

The Rust API will start on:
- `http://127.0.0.1:8098`

### Terminal 3, start the web app
```bash
cd apps/web
npm run dev -- --host 0.0.0.0 --port 5173
```

The web app will start on:
- `http://127.0.0.1:5173`

## Run with Docker Compose
```bash
docker compose up --build
```

This starts:
- Rust API on `http://127.0.0.1:8098`
- Python optimizer on `http://127.0.0.1:8099`

## Test the backend quickly

### Health check
```bash
curl http://127.0.0.1:8099/health
```

### List templates
```bash
curl http://127.0.0.1:8098/templates
```

### Create an optimization job
```bash
curl -X POST http://127.0.0.1:8098/jobs/optimize \
  -H 'Content-Type: application/json' \
  -d '{
    "template": "boutique-wealth-desk",
    "solver_mode": "hybrid",
    "annual_capital_gbp": 25000000
  }'
```

### Upload a CSV file
```bash
curl -X POST http://127.0.0.1:8098/uploads/csv \
  -F 'file=@examples/boutique-wealth-desk.csv'
```

## Current state
This repo contains a runnable local demo scaffold with:
- simulator-first optimization workflows
- Rust API endpoints for scenarios, jobs, results, templates, and CSV upload scaffold
- frontend shell for scenario building and result comparisons
- shared schemas and example datasets
- CI workflow and local bootstrap support

## Key docs
- `docs/onboarding-analysts.md`
- `docs/onboarding-quants.md`
- `docs/onboarding-developers.md`
- `docs/architecture.md`
- `docs/api.md`
