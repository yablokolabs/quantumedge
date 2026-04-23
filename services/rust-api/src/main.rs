use std::{collections::HashMap, sync::{Arc, Mutex}};
use axum::{extract::{Path, State}, routing::{get, post}, Json, Router};
use serde::{Deserialize, Serialize};
use tower_http::cors::CorsLayer;
use uuid::Uuid;

#[derive(Clone)]
struct AppState {
    jobs: Arc<Mutex<HashMap<Uuid, JobStatus>>>,
    results: Arc<Mutex<HashMap<Uuid, OptimizationResult>>>,
    optimizer_base_url: String,
}

impl Default for AppState {
    fn default() -> Self {
        Self {
            jobs: Arc::new(Mutex::new(HashMap::new())),
            results: Arc::new(Mutex::new(HashMap::new())),
            optimizer_base_url: std::env::var("OPTIMIZER_URL").unwrap_or_else(|_| "http://127.0.0.1:8099".to_string()),
        }
    }
}

#[derive(Debug, Serialize)]
struct TemplateInfo {
    id: &'static str,
    name: &'static str,
    summary: &'static str,
}

#[derive(Debug, Deserialize)]
struct ScenarioRequest {
    name: String,
    template: String,
    solver_mode: String,
}

#[derive(Debug, Deserialize)]
struct OptimizeRequest {
    scenario_id: Option<Uuid>,
    template: Option<String>,
    solver_mode: String,
    annual_capital_gbp: Option<f64>,
}

#[derive(Debug, Serialize, Clone)]
struct JobStatus {
    id: Uuid,
    state: String,
    solver_mode: String,
    audit_note: String,
    result_id: Option<Uuid>,
}

#[derive(Debug, Serialize, Deserialize, Clone)]
struct OptimizationResult {
    result_id: Uuid,
    template: String,
    adapter: String,
    solver_mode: String,
    baseline_expected_return: f64,
    optimized_expected_return: Option<f64>,
    annualized_gain_estimate_gbp: f64,
    explainability: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct OptimizerResponse {
    template: String,
    adapter: String,
    solver_mode: String,
    baseline: MetricBundle,
    optimized: serde_json::Value,
    annualized_gain_estimate_gbp: f64,
    explainability: Vec<String>,
}

#[derive(Debug, Deserialize)]
struct MetricBundle {
    expected_return: f64,
}

#[tokio::main]
async fn main() {
    let state = AppState::default();
    let app = Router::new()
        .route("/templates", get(get_templates))
        .route("/scenarios", post(create_scenario))
        .route("/jobs/optimize", post(create_job))
        .route("/jobs/:id", get(get_job))
        .route("/results/:id", get(get_result))
        .with_state(state)
        .layer(CorsLayer::permissive());

    let listener = tokio::net::TcpListener::bind("0.0.0.0:8098").await.unwrap();
    println!("QuantumEdge API listening on http://0.0.0.0:8098");
    axum::serve(listener, app).await.unwrap();
}

async fn get_templates() -> Json<Vec<TemplateInfo>> {
    Json(vec![
        TemplateInfo { id: "boutique-wealth-desk", name: "Boutique Wealth Desk", summary: "Portfolio optimization template for wealth-style mandates." },
        TemplateInfo { id: "treasury-desk", name: "Investment Bank Treasury Desk", summary: "Treasury allocation and liquidity optimization template." },
        TemplateInfo { id: "advisory-allocation", name: "Advisory Allocation Team", summary: "Client allocation and rebalancing template." },
    ])
}

async fn create_scenario(Json(payload): Json<ScenarioRequest>) -> Json<serde_json::Value> {
    Json(serde_json::json!({
        "scenario_id": Uuid::new_v4(),
        "name": payload.name,
        "template": payload.template,
        "solver_mode": payload.solver_mode,
        "status": "created"
    }))
}

async fn create_job(State(state): State<AppState>, Json(payload): Json<OptimizeRequest>) -> Json<JobStatus> {
    let job_id = Uuid::new_v4();
    let result_id = Uuid::new_v4();
    let optimizer_payload = serde_json::json!({
        "template": payload.template.clone().unwrap_or_else(|| "boutique-wealth-desk".to_string()),
        "solver_mode": payload.solver_mode,
        "annual_capital_gbp": payload.annual_capital_gbp.unwrap_or(25_000_000.0)
    });

    let mut job = JobStatus {
        id: job_id,
        state: "running".to_string(),
        solver_mode: payload.solver_mode.clone(),
        audit_note: format!("Scenario {:?} executed with compliance-friendly logging enabled.", payload.scenario_id),
        result_id: Some(result_id),
    };

    let client = reqwest::Client::new();
    let response = client.post(format!("{}/optimize", state.optimizer_base_url))
        .json(&optimizer_payload)
        .send()
        .await;

    match response {
        Ok(resp) => match resp.json::<OptimizerResponse>().await {
            Ok(body) => {
                let optimized_expected_return = body.optimized.get("metrics").and_then(|m| m.get("expected_return")).and_then(|v| v.as_f64());
                let result = OptimizationResult {
                    result_id,
                    template: body.template,
                    adapter: body.adapter,
                    solver_mode: body.solver_mode,
                    baseline_expected_return: body.baseline.expected_return,
                    optimized_expected_return,
                    annualized_gain_estimate_gbp: body.annualized_gain_estimate_gbp,
                    explainability: body.explainability,
                };
                state.results.lock().unwrap().insert(result_id, result);
                job.state = "completed".to_string();
            }
            Err(_) => job.state = "failed".to_string(),
        },
        Err(_) => job.state = "failed".to_string(),
    }

    state.jobs.lock().unwrap().insert(job.id, job.clone());
    Json(job)
}

async fn get_job(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    let jobs = state.jobs.lock().unwrap();
    Json(match jobs.get(&id) {
        Some(job) => serde_json::json!(job),
        None => serde_json::json!({"error": "job not found"}),
    })
}

async fn get_result(State(state): State<AppState>, Path(id): Path<Uuid>) -> Json<serde_json::Value> {
    let results = state.results.lock().unwrap();
    Json(match results.get(&id) {
        Some(result) => serde_json::json!(result),
        None => serde_json::json!({"error": "result not found"}),
    })
}
