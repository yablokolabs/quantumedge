from fastapi import FastAPI
from pydantic import BaseModel
from .optimizer import run_demo_optimization

app = FastAPI(title="QuantumEdge Optimizer")

class OptimizeRequest(BaseModel):
    template: str = "boutique-wealth-desk"
    solver_mode: str = "simulator"
    annual_capital_gbp: float = 25_000_000

@app.get("/health")
def health():
    return {"status": "ok", "service": "python-optimizer"}

@app.post("/optimize")
def optimize(req: OptimizeRequest):
    return run_demo_optimization(req.template, req.solver_mode, req.annual_capital_gbp)
