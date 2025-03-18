use axum::http::StatusCode;
use oysters_core::Oyster;

use axum::extract::Path;
use axum::response::IntoResponse;
use axum::routing::{delete, get, post};
use axum::{Extension, Json, Router};
use tower_http::trace::{self, TraceLayer};
use tracing::{Level, info};

use std::sync::Arc;
use tokio::sync::RwLock;

mod config;

type Map = Oyster<String, String>;
type MapState = Arc<RwLock<Map>>;

/// Get a value given its `key`.
pub async fn get_value(
    Path(key): Path<String>,
    Extension(map): Extension<MapState>,
) -> impl IntoResponse {
    let reader = map.read().await;
    if let Some(v) = reader.get(&key) {
        let v = v.clone(); // we need to clone v so that it is no longer a reference to a value under reader
        drop(reader);
        map.write().await.update_resource_descriptor(&key);
        (StatusCode::OK, v)
    } else {
        (StatusCode::NOT_FOUND, String::new())
    }
}

/// Get a full value given its `key`.
pub async fn get_full_value(
    Path(key): Path<String>,
    Extension(map): Extension<MapState>,
) -> impl IntoResponse {
    if let Some(v) = map.read().await.get_full(&key) {
        (
            StatusCode::OK,
            format!("Value: {}\nLast Used: {}", v.0, v.1.used),
        )
    } else {
        (StatusCode::NOT_FOUND, String::new())
    }
}

/// Filter all.
pub async fn filter_all(Extension(map): Extension<MapState>, pattern: String) -> impl IntoResponse {
    let reader = map.read().await;
    let values = reader.filter(&pattern);
    let mut owned_values = Vec::new();

    for value in values {
        owned_values.push((value.0.to_owned(), value.1.to_owned()));
    }

    Json(owned_values)
}

/// Filter all by keys.
pub async fn filter_keys(
    Extension(map): Extension<MapState>,
    pattern: String,
) -> impl IntoResponse {
    let reader = map.read().await;
    let values = reader.filter_keys(&pattern);
    let mut owned_values = Vec::new();

    for value in values {
        owned_values.push(value.to_owned());
    }

    Json(owned_values)
}

/// Insert a key.
pub async fn insert_value(
    Path(key): Path<String>,
    Extension(map): Extension<MapState>,
    value: String,
) -> impl IntoResponse {
    map.write().await.insert(key, value);
}

/// Increment a key.
pub async fn incr_value(
    Path(key): Path<String>,
    Extension(map): Extension<MapState>,
) -> impl IntoResponse {
    map.write().await.incr(key);
}

/// Decrement a key.
pub async fn decr_value(
    Path(key): Path<String>,
    Extension(map): Extension<MapState>,
) -> impl IntoResponse {
    map.write().await.decr(key);
}

/// Dump the map to a database.
pub async fn dump(Extension(map): Extension<MapState>) -> impl IntoResponse {
    tokio::task::spawn(async move {
        map.read().await.dump().unwrap();
    });

    "Data dump in started"
}

/// Scan the map for old values and remove them.
pub async fn scan(Extension(map): Extension<MapState>) -> impl IntoResponse {
    map.write().await.scan_sync();
}

/// Remove a key.
pub async fn remove_value(
    Path(key): Path<String>,
    Extension(map): Extension<MapState>,
) -> impl IntoResponse {
    map.write().await.remove(&key);
}

#[tokio::main]
async fn main() {
    tracing_subscriber::fmt()
        .with_target(false)
        .compact()
        .init();

    let mut map: Map = Oyster::new();
    map.restore().unwrap();

    let config = config::Config::get_config();

    let app = Router::new()
        .route("/_dump", post(dump))
        .route("/_scan", post(scan))
        .route("/_full/{key}", get(get_full_value))
        .route("/_filter", post(filter_all))
        .route("/_filter/keys", post(filter_keys))
        .route("/_incr/{key}", post(incr_value))
        .route("/_decr/{key}", post(decr_value))
        .route("/{key}", get(get_value))
        .route("/{key}", post(insert_value))
        .route("/{key}", delete(remove_value))
        .layer(Extension(Arc::new(RwLock::new(map))))
        .layer(
            TraceLayer::new_for_http()
                .make_span_with(trace::DefaultMakeSpan::new().level(Level::INFO))
                .on_response(trace::DefaultOnResponse::new().level(Level::INFO)),
        );

    let listener = tokio::net::TcpListener::bind(format!("0.0.0.0:{}", config.port))
        .await
        .unwrap();

    info!("🦪 OYSTERS");
    info!("listening on http://localhost:{}", config.port);
    axum::serve(listener, app).await.unwrap();
}
