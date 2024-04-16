use actix_web::{web, App, HttpResponse, HttpServer, Responder};
use serde::{Deserialize, Serialize};
use std::sync::{Arc, Mutex};
use std::collections::HashMap;
use uuid::Uuid;
use actix_web::http::StatusCode;

use std::env;





#[derive(Serialize, Deserialize, Clone, PartialEq, Debug)]
struct TodoItem {
    id: Uuid,
    description: String,
    completed: bool,
}

struct AppState {
    todo_items: Mutex<HashMap<Uuid, TodoItem>>,
}

impl AppState {
    pub fn new() -> Self {
        AppState {
            todo_items: Mutex::new(HashMap::new()),
        }
    }

    pub fn add_item(&self, item: TodoItem) {
        let mut items = self.todo_items.lock().unwrap();
        items.insert(item.id, item);
    }

    pub fn get_item(&self, id: Uuid) -> Option<TodoItem> {
        let items = self.todo_items.lock().unwrap();
        items.get(&id).cloned()
    }

    pub fn remove_item(&self, id: Uuid) -> Option<TodoItem> {
        let mut items = self.todo_items.lock().unwrap();
        items.remove(&id)
    }

    pub fn update_item(&self, id: Uuid, item: TodoItem) -> Option<TodoItem> {
        let mut items = self.todo_items.lock().unwrap();
        items.insert(id, item)
    }
}

async fn add_todo(data: web::Data<Arc<AppState>>, item: web::Json<TodoItem>) -> impl Responder {
    let new_item = TodoItem {
        id: Uuid::new_v4(),
        description: item.description.clone(),
        completed: item.completed,
    };

    data.add_item(new_item.clone());
    HttpResponse::Ok().json(new_item)
}

async fn get_todo(data: web::Data<Arc<AppState>>, id: web::Path<Uuid>) -> impl Responder {
    if let Some(item) = data.get_item(id.into_inner()) {
        HttpResponse::Ok().json(item)
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn delete_todo(data: web::Data<Arc<AppState>>, id: web::Path<Uuid>) -> impl Responder {
    if data.remove_item(id.into_inner()).is_some() {
        HttpResponse::Ok().finish()
    } else {
        HttpResponse::NotFound().finish()
    }
}

async fn update_todo(data: web::Data<Arc<AppState>>, id: web::Path<Uuid>, item: web::Json<TodoItem>) -> impl Responder {
    let updated_item = TodoItem {
        id: id.into_inner(), // Capture UUID once here
        description: item.description.clone(),
        completed: item.completed,
    };

    if data.update_item(updated_item.id, updated_item.clone()).is_some() {
        HttpResponse::Ok().json(updated_item)
    } else {
        HttpResponse::NotFound().finish()
    }
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let app_state = web::Data::new(Arc::new(AppState::new()));
    std::env::set_var("RUST_LOG", "debug");

    env_logger::init();

    HttpServer::new(move || {
        App::new()
            .app_data(app_state.clone())
            .route("/todos", web::post().to(add_todo))
            .route("/todos/{id}", web::get().to(get_todo))
            .route("/todos/{id}", web::delete().to(delete_todo))
            .route("/todos/{id}", web::put().to(update_todo))
    })
    .bind("127.0.0.1:8080")?
    .run()
    .await
}








// tests:


#[cfg(test)]
mod integration_tests {
    use super::*;
    use actix_web::{test, web, App, http::StatusCode};

    #[actix_rt::test]
    async fn test_add_todo() {
        env::set_var("RUST_LOG", "debug");
env_logger::init(); 
        let app = test::init_service(
            App::new()
                .data(web::Data::new(AppState::new()))
                .route("/todos", web::post().to(add_todo))
        ).await;
    
        let new_todo = TodoItem {
            id: Uuid::new_v4(),
            description: "Integration test todo".to_string(),
            completed: false,
        };
    
        let req = test::TestRequest::post()
            .uri("/todos")
            .set_json(&new_todo)
            .to_request();
        let resp = test::call_service(&app, req).await;
    
        // First check the status before consuming the response
        assert_eq!(resp.status(), StatusCode::OK, "Expected OK but got {}", resp.status());
    
        // Then read the response body if needed (this consumes resp)
        let body = test::read_body(resp).await;
        println!("Response body: {:?}", std::str::from_utf8(&body));
    }
    

    #[actix_rt::test]
    async fn test_get_todo() {
        let data = web::Data::new(AppState::new());
        let test_item = TodoItem {
            id: Uuid::new_v4(),
            description: "Test get todo".to_string(),
            completed: true,
        };

        data.add_item(test_item.clone());
        let app = test::init_service(
            App::new()
                .app_data(data.clone())
                .route("/todos/{id}", web::get().to(get_todo))
        ).await;

        let req = test::TestRequest::get()
            .uri(&format!("/todos/{}", test_item.id))
            .to_request();
        let resp = test::call_service(&app, req).await;

        assert_eq!(resp.status(), StatusCode::OK);
        let resp_item: TodoItem = test::read_body_json(resp).await;
        assert_eq!(resp_item, test_item);
    }

    #[actix_rt::test]
    async fn test_delete_todo() {
        let data = web::Data::new(AppState::new());
        let test_item = TodoItem {
            id: Uuid::new_v4(),
            description: "Test delete todo".to_string(),
            completed: false,
        };

        data.add_item(test_item.clone());
        let app = test::init_service(App::new()).await;


}}
