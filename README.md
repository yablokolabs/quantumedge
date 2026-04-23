# QuantumEdge

QuantumEdge is a production-oriented optimization platform for small and mid-sized UK investment banks. It is designed to help desks, treasury teams, and portfolio managers compare classical baselines against quantum-ready optimization workflows for portfolio construction, rebalancing, treasury allocation, and trade basket selection.

## Positioning
QuantumEdge sells business outcomes, not quantum hype.

It is built to answer questions like:
- can we improve allocation quality versus a classical baseline?
- can we reduce turnover or capital friction under realistic constraints?
- can we prepare today on simulators and hybrid solvers before moving to hardware-backed workflows later?

## Monorepo layout
- `apps/web` — analyst-facing web app
- `services/rust-api` — auth, workspace, jobs, audit, orchestration API
- `services/python-optimizer` — optimization engine with simulator and hybrid adapters
- `packages/shared-schemas` — shared API and scenario schemas
- `examples` — sample datasets and demo scenarios
- `docs` — architecture, onboarding, and API docs

## Current state
This repo contains a runnable local demo scaffold with:
- simulator-first optimization workflows
- Rust API endpoints for scenarios, jobs, results, and templates
- frontend shell for scenario building and result comparisons
- shared schemas and example datasets

## Quick start
See:
- `docs/onboarding-analysts.md`
- `docs/onboarding-quants.md`
- `docs/onboarding-developers.md`
- `docs/architecture.md`
