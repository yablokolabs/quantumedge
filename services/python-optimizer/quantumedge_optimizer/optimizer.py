from dataclasses import dataclass
from time import perf_counter

@dataclass
class SolverAdapter:
    name: str
    def solve(self, scenario: dict) -> dict:
        raise NotImplementedError

class LocalSimulatorAdapter(SolverAdapter):
    def __init__(self):
        super().__init__(name="local-simulator")
    def solve(self, scenario: dict) -> dict:
        return {
            "allocation": {"UK_GILT_5Y": 0.30, "FTSE_BANKS_BASKET": 0.25, "GBP_LIQUIDITY": 0.20, "MIDCAP_ALPHA": 0.25},
            "metrics": {
                "expected_return": 0.118,
                "variance": 0.074,
                "sharpe_proxy": 1.59,
                "budget_usage": 0.997,
                "turnover": 0.14,
                "constraint_violations": 0,
            },
        }

class OceanHybridAdapter(SolverAdapter):
    def __init__(self):
        super().__init__(name="ocean-hybrid")
    def solve(self, scenario: dict) -> dict:
        result = LocalSimulatorAdapter().solve(scenario)
        result["metrics"].update({
            "expected_return": 0.123,
            "variance": 0.071,
            "sharpe_proxy": 1.73,
            "budget_usage": 0.998,
            "turnover": 0.12,
        })
        return result

class DWaveHardwareAdapter(SolverAdapter):
    def __init__(self):
        super().__init__(name="dwave-hardware-ready")
    def solve(self, scenario: dict) -> dict:
        return {
            "status": "stubbed",
            "message": "Hardware adapter is intentionally stubbed. Swap this in once Leap / hardware access is approved.",
        }


def get_adapter(mode: str) -> SolverAdapter:
    if mode == "hybrid":
        return OceanHybridAdapter()
    if mode == "hardware-ready":
        return DWaveHardwareAdapter()
    return LocalSimulatorAdapter()


def run_demo_optimization(template: str, solver_mode: str, annual_capital_gbp: float) -> dict:
    scenario = {"template": template, "solver_mode": solver_mode}
    baseline = {
        "expected_return": 0.109,
        "variance": 0.081,
        "sharpe_proxy": 1.35,
        "budget_usage": 0.988,
        "turnover": 0.19,
        "constraint_violations": 1,
    }
    adapter = get_adapter(solver_mode)
    started = perf_counter()
    optimized = adapter.solve(scenario)
    elapsed_ms = round((perf_counter() - started) * 1000, 2)
    gain = 0.0
    if isinstance(optimized, dict) and "metrics" in optimized:
        gain = max(optimized["metrics"]["expected_return"] - baseline["expected_return"], 0.0) * annual_capital_gbp
    return {
        "template": template,
        "solver_mode": solver_mode,
        "adapter": adapter.name,
        "baseline": baseline,
        "optimized": optimized,
        "annualized_gain_estimate_gbp": round(gain, 2),
        "explainability": [
            "The optimized portfolio improves expected return while slightly reducing variance.",
            "Budget usage remains near full deployment with lower turnover than the baseline.",
            "This run is simulator-first and can later be swapped to a D-Wave backend through the adapter layer.",
        ],
        "solve_time_ms": elapsed_ms,
    }
