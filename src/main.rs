use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};

#[derive(Clone, Debug, Serialize, Deserialize)]
struct Task {
    id: u64,
    title: String,
    completed: bool,
}

#[derive(Clone, Default)]
struct AppState {
    tasks: Arc<Mutex<Vec<Task>>>,
}

#[derive(Debug, Deserialize)]
struct CreateTask {
    title: String,
}

#[get("/health")]
async fn health() -> impl Responder {
    HttpResponse::Ok().json(serde_json::json!({"status": "healthy", "service": "Rust-CLI-Task-Manager"}))
}

#[get("/api/v1/tasks")]
async fn list_tasks(state: web::Data<AppState>) -> impl Responder {
    match state.tasks.lock() {
        Ok(tasks) => HttpResponse::Ok().json(tasks.clone()),
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

async fn create_task(state: web::Data<AppState>, payload: web::Json<CreateTask>) -> impl Responder {
    let title = payload.title.trim();
    if title.is_empty() || title.len() > 500 {
        return HttpResponse::BadRequest().json(serde_json::json!({"error": "title must contain 1-500 characters"}));
    }
    match state.tasks.lock() {
        Ok(mut tasks) => {
            let id = tasks.iter().map(|task| task.id).max().unwrap_or(0) + 1;
            let task = Task { id, title: title.to_owned(), completed: false };
            tasks.push(task.clone());
            HttpResponse::Created().json(task)
        }
        Err(_) => HttpResponse::InternalServerError().finish(),
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState::default();
    HttpServer::new(move || {
        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(health)
            .service(list_tasks)
            .route("/api/v1/tasks", web::post().to(create_task))
    })
    .bind(("0.0.0.0", 8080))?
    .run()
    .await
}
