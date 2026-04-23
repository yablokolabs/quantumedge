# QuantumEdge API

## Rust API endpoints

### POST /scenarios
Create a scenario envelope.

Example payload:
```json
{
  "name": "UK Treasury Pilot",
  "template": "treasury-desk",
  "solver_mode": "simulator"
}
```

### POST /jobs/optimize
Submit an optimization job.

Example payload:
```json
{
  "template": "boutique-wealth-desk",
  "solver_mode": "hybrid",
  "annual_capital_gbp": 25000000
}
```

### GET /jobs/:id
Return job state and audit note.

### GET /results/:id
Return optimization result summary.

### GET /templates
Return built-in scenario templates.
