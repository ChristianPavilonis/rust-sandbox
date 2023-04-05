pub mod auth;
pub mod entity;
pub mod middleware;
pub mod todos;

use actix_web::{get, web, App, HttpResponse, HttpServer, Responder};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use actix_web_httpauth::middleware::HttpAuthentication;
use migration::{Migrator, MigratorTrait};
use sea_orm::{entity::*, Database, DatabaseConnection};
use std::env;

#[derive(Debug, Clone)]
pub struct AppState {
    db: DatabaseConnection,
}

#[get("/")]
async fn home() -> impl Responder {
    HttpResponse::Ok().body("Hello")
}

#[get("/protected")]
async fn protected(auth: BearerAuth) -> impl Responder {
    let token = auth::decode_jwt(auth.token()).unwrap();
    let user_id = token.sub;

    HttpResponse::Ok().body("welcome to the club")
}

async fn connect_to_db() -> std::io::Result<DatabaseConnection> {
    let path = env::current_dir()?;

    let db = Database::connect(format!("sqlite:{}/database.sqlite", path.display()))
        .await
        .unwrap();

    Ok(db)
}

#[actix_web::main]
async fn main() -> std::io::Result<()> {
    let state = AppState {
        db: connect_to_db().await?,
    };

    HttpServer::new(move || {
        let auth = HttpAuthentication::bearer(auth::verify_jwt);

        App::new()
            .app_data(web::Data::new(state.clone()))
            .service(home)
            .service(auth::register_user)
            .service(auth::login)
            .service(
                web::scope("/api")
                    .wrap(auth)
                    .service(protected)
                    .service(todos::create_todo)
                    .service(todos::get_todos)
                    .service(todos::complete_todo),
            )
    })
    .bind(("127.0.0.1", 8080))?
    .run()
    .await
}
