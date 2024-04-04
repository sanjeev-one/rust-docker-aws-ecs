use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::collections::HashMap;
use uuid::Uuid;

#[derive(Serialize, Deserialize, Clone)]
struct TodoItem {
    id: Uuid,
    description: String,
    completed: bool,
}

#[derive(Clone)]
struct AppState {
    todo_items: HashMap<Uuid, TodoItem>,
}

impl AppState {
    fn new() -> AppState {
        AppState {
            todo_items: HashMap::new(),
        }
    }

    fn add_item(&mut self, item: TodoItem) {
        self.todo_items.insert(item.id, item);
    }

    fn get_item(&self, id: Uuid) -> Option<&TodoItem> {
        self.todo_items.get(&id)
    }

    fn remove_item(&mut self, id: Uuid) -> Option<TodoItem> {
        self.todo_items.remove(&id)
    }

    fn update_item(&mut self, id: Uuid, item: TodoItem) -> Option<TodoItem> {
        self.todo_items.insert(id, item)
    }
}

async fn add_todo(data: web::Data<AppState>, item: web::Json<TodoItem>) -> impl Responder {
    let mut state = data.into_inner();
    let new_item = TodoItem {
        id: Uuid::new_v4(),
        description: item.description.clone(),
        completed: item.completed,
    };

    state.add_item(new_item.clone());
    HttpResponse::Ok().json(new_item)
}

async fn get_todo(data: web::Data<AppState>, web::Path(id): web::Path<Uuid>) -> impl Responder {
    let state = data.into_inner();
    if let Some(item) = state.get_item(id) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_todo(data: web::Data<AppState>, web::Path(id): web::Path<Uuid>) -> impl Responder {
    let mut state = data.into_inner();
    if state.remove_item(id).is_some() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn update_todo(data: web::Data<AppState>, web::Path(id): web::Path<Uuid>, item: web::Json<TodoItem>) -> impl Responder {
    let mut state = data.into_inner();
    let updated_item = TodoItem {
        id,
        description: item.description.clone(),
        completed: item.completed,
    };

    if state.update_item(id, updated_item.clone()).is_some() {
        HttpResponse::Ok().json(updated_item)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(AppState::new());

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/todos", web::post().to(add_todo))
            .route("/todos/{id}", web::get().to(get_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
            .route("/todos/{id}", web::put().to(update_todo))
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
