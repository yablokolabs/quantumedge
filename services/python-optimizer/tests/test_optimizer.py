from quantumedge_optimizer.optimizer import run_demo_optimization


def test_demo_optimization_returns_comparison():
    result = run_demo_optimization("boutique-wealth-desk", "simulator")
    assert result["baseline"]["expected_return"] < result["optimized"]["metrics"]["expected_return"]
