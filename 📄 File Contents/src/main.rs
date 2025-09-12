use axum::{
    extract::{Json, State},
    routing::{get, post},
    Router,
};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::{collections::HashMap, net::SocketAddr, sync::Arc};
use tokio::sync::Mutex;
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone)]
struct Event {
    id: String,
    event_type: String,
    user_id: String,
    timestamp: String,
    metadata: serde_json::Value,
}

type EventStore = Arc<Mutex<Vec<Event>>>;

#[tokio::main]
async fn main() {
    let store: EventStore = Arc::new(Mutex::new(Vec::new()));

    let app = Router::new()
        .route("/track", post(track_event))
        .route("/metrics", get(get_metrics))
        .with_state(store.clone());

    let addr = SocketAddr::from(([127, 0, 0, 1], 4700));
    println!("📊 Analytics Service running at http://{}", addr);

    axum::Server::bind(&addr)
        .serve(app.into_make_service_with_state(store))
        .await
        .unwrap();
}

async fn track_event(State(store): State<EventStore>, Json(input): Json<Event>) -> Json<Event> {
    let id = Uuid::new_v4().to_string();
    let event = Event {
        id,
        event_type: input.event_type,
        user_id: input.user_id,
        timestamp: Utc::now().to_rfc3339(),
        metadata: input.metadata,
    };

    let mut events = store.lock().await;
    events.push(event.clone());

    Json(event)
}

async fn get_metrics(State(store): State<EventStore>) -> Json<HashMap<String, usize>> {
    let events = store.lock().await;
    let mut counts = HashMap::new();

    for event in events.iter() {
        *counts.entry(event.event_type.clone()).or_insert(0) += 1;
    }

    Json(counts)
}
