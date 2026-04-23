from quantumedge_optimizer.optimizer import run_demo_optimization


def test_demo_optimization_returns_comparison():
    result = run_demo_optimization("boutique-wealth-desk", "simulator", 25_000_000)
    assert result["baseline"]["expected_return"] < result["optimized"]["metrics"]["expected_return"]
    assert result["annualized_gain_estimate_gbp"] > 0
