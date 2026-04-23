# QuantumEdge Architecture

```text
                +---------------------------+
                |      apps/web (React)     |
                | dashboard, scenarios, UI  |
                +------------+--------------+
                             |
                             v
                +---------------------------+
                |   services/rust-api       |
                | auth, workspaces, jobs,   |
                | audit, reports, features  |
                +------------+--------------+
                             |
                 REST + internal job control
                             |
                             v
                +---------------------------+
                | services/python-optimizer |
                | QUBO/BQM/CQM adapters,    |
                | simulator + hybrid flows  |
                +------------+--------------+
                             |
                             v
                +---------------------------+
                | solver backends           |
                | local sim / hybrid /      |
                | hardware-ready stub       |
                +---------------------------+
```

## Design principles
- simulator first
- hardware as pluggable backend
- clear baseline vs optimized comparison
- compliance-friendly audit logging
- premium but practical institutional workflow
