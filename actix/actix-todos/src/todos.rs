use actix_web::{
    get, post,
    web::{Data, Json, Path},
    HttpResponse, Responder, patch,
};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use sea_orm::{ActiveModelTrait, ColumnTrait, EntityTrait, QueryFilter, Set};
use serde_json::json;

use crate::{
    auth::id_from_extractor, entity::prelude::Todo, entity::todo, entity::todo::Column::*, AppState,
};

#[derive(serde::Deserialize)]
pub struct TodoRequest {
    name: String,
}

#[post("/todos")]
pub async fn create_todo(
    input: Json<TodoRequest>,
    auth: BearerAuth,
    state: Data<AppState>,
) -> impl Responder {
    let user_id = id_from_extractor(auth);

    let todo = todo::ActiveModel {
        name: Set(input.name.clone()),
        user_id: Set(user_id as i32),
        status: Set(false),
        ..Default::default()
    };

    let todo = todo.save(&state.db).await.unwrap();

    HttpResponse::Ok().body(
        json!({
            "id": todo.id.unwrap()
        })
        .to_string(),
    )
}

#[derive(serde::Serialize, serde::Deserialize)]
pub struct RepsonseTodo {
    name: String,
    id: i32,
    status: bool,
}

#[get("/todos")]
pub async fn get_todos(auth: BearerAuth, state: Data<AppState>) -> impl Responder {
    let user_id = id_from_extractor(auth);

    let todos: Vec<_> = Todo::find()
        .filter(UserId.eq(user_id))
        .all(&state.db)
        .await
        .unwrap()
        .iter()
        .map(|todo| RepsonseTodo {
            name: todo.name.clone(),
            id: todo.id,
            status: todo.status,
        }).collect();

    HttpResponse::Ok().body(json!({ "todos": todos }).to_string())
}

#[patch("/todos/{id}")]
pub async fn complete_todo(path: Path<i32>, state: Data<AppState>) -> impl Responder {
    let id = path.into_inner();

    let todo = Todo::find_by_id(id).one(&state.db).await.unwrap();

    match todo {
        Some(todo) => {
            let mut todo: todo::ActiveModel = todo.into();
            todo.status = Set(true);

            todo.update(&state.db).await.unwrap();
        }
        None => {},
    }


    HttpResponse::NoContent()
}
