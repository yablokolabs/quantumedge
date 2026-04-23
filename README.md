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

## Current state
This repo contains a runnable local demo scaffold with:
- simulator-first optimization workflows
- Rust API endpoints for scenarios, jobs, results, templates, and CSV upload scaffold
- frontend shell for scenario building and result comparisons
- shared schemas and example datasets
- CI workflow and local bootstrap support

## Quick start
```bash
./scripts/bootstrap.sh
```

Then use:
- Rust API on `http://localhost:8098`
- Python optimizer on `http://localhost:8099`
- web app from `apps/web`

## Key docs
- `docs/onboarding-analysts.md`
- `docs/onboarding-quants.md`
- `docs/onboarding-developers.md`
- `docs/architecture.md`
- `docs/api.md`
