use actix_web::{post, web, App, HttpResponse, HttpServer, Responder};
use chrono::Utc;
use serde::{Deserialize, Serialize};
use serde_json::Value;

static PING_BASE: &str = r#"{"message":"wagmi","lang":"Rust"}"#;

#[derive(Deserialize)]
struct WagmiRequest {
    a: Option<Value>,
    b: Option<Value>,
}

#[derive(Serialize)]
struct AddResponse {
    result: f64,
    a: f64,
    b: f64,
    status: String,
}

#[derive(Serialize)]
struct ErrorResponse {
    error: String,
}

#[post("/wagmi")]
async fn wagmi_handler(body: web::Json<WagmiRequest>) -> impl Responder {
    // Ping request: no fields or empty object
    if body.a.is_none() && body.b.is_none() {
        let timestamp = Utc::now().to_rfc3339();
        let response = format!(r#"{{"message":"wagmi","timestamp":"{}","lang":"Rust"}}"#, timestamp);
        return HttpResponse::Ok().body(response);
    }

    // Addition request: both a and b must be present
    if let (Some(a_val), Some(b_val)) = (&body.a, &body.b) {
        if let (Some(a), Some(b)) = (a_val.as_f64(), b_val.as_f64()) {
            if a >= 0.0 && b >= 0.0 && a + b <= 100.0 {
                let response = AddResponse {
                    result: a + b,
                    a,
                    b,
                    status: "success".to_string(),
                };
                return HttpResponse::Ok().json(response);
            }
        }
    }

    HttpResponse::BadRequest().json(ErrorResponse {
        error: "Invalid input".to_string(),
    })
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    //dynamic port
    let port = std::env::var("PORT").unwrap_or_else(|_| "8000".to_string());
    println!("Server running on port {}", port);

    HttpServer::new(|| App::new().service(wagmi_handler))
        .workers(2)
        .max_connections(5000)
        .bind(format!("0.0.0.0:{}", port))?
        .run()
        .await
}