use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use std::time::SystemTime;
use tokio::sync::RwLock;
use once_cell::sync::Lazy;

// Cached timestamp for ping responses
struct TimestampCache {
    timestamp: RwLock<String>,
    last_updated: RwLock<SystemTime>,
}

static TIMESTAMP_CACHE: Lazy<TimestampCache> = Lazy::new(|| TimestampCache {
    timestamp: RwLock::new(String::new()),
    last_updated: RwLock::new(SystemTime::UNIX_EPOCH),
});

// Ping response struct
#[derive(Serialize)]
struct PingResponse {
    message: &'static str,
    timestamp: String,
    lang: &'static str,
}

// Request struct - using serde instead of sonic-rs
#[derive(Deserialize)]
struct WagmiRequest {
    a: Option<f64>,
    b: Option<f64>,
}

// Response structs
#[derive(Serialize)]
struct AddResponse {
    result: f64,
    a: f64,
    b: f64,
    status: &'static str,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: &'static str,
}

#[post("/wagmi")]
async fn wagmi_handler(body: web::Json<WagmiRequest>) -> impl Responder {
    // Update timestamp cache (every 1ms)
    let now = SystemTime::now();
    {
        let last_updated = *TIMESTAMP_CACHE.last_updated.read().await;
        if now.duration_since(last_updated).map_or(true, |d| d.as_millis() >= 1) {
            let mut timestamp = TIMESTAMP_CACHE.timestamp.write().await;
            let mut last_updated = TIMESTAMP_CACHE.last_updated.write().await;
            *timestamp = Utc::now().to_rfc3339();
            *last_updated = now;
        }
    }

    // Ping request
    if body.a.is_none() && body.b.is_none() {
        let timestamp = TIMESTAMP_CACHE.timestamp.read().await;
        let response = PingResponse {
            message: "wagmi",
            timestamp: timestamp.clone(),
            lang: "Rust",
        };
        return HttpResponse::Ok().json(response);
    }

    // Addition request
    if let (Some(a), Some(b)) = (body.a, body.b) {
        if a >= 0.0 && b >= 0.0 && a + b <= 100.0 {
            let response = AddResponse {
                result: a + b,
                a,
                b,
                status: "success",
            };
            return HttpResponse::Ok().json(response);
        }
    }

    // Invalid input
    HttpResponse::BadRequest().json(ErrorResponse {
        error: "Invalid input",
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());

    HttpServer::new(|| {
        App::new()
            .service(wagmi_handler)
            .default_service(web::route().to(HttpResponse::NotFound))
    })
    .workers(1)
    .max_connections(6000)
    .bind(format!("[::]:{}", port))?
    .run()
    .await
}