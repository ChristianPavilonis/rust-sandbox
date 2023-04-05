use crate::{entity::prelude::User, entity::user::ActiveModel, entity::user::Column::*, AppState};
use actix_web::{dev::ServiceRequest, *};
use actix_web_httpauth::extractors::bearer::BearerAuth;
use bcrypt::{hash, verify, DEFAULT_COST};
use jsonwebtoken::{decode, encode, Algorithm, DecodingKey, EncodingKey, Header, Validation};
use sea_orm::*;
use serde::{Deserialize, Serialize};
use serde_json::json;
use std::time::{SystemTime, UNIX_EPOCH};

// Claims for JWT
#[derive(Debug, Serialize, Deserialize)]
pub struct Claims {
    pub sub: u32,
    exp: usize,
}

#[derive(serde::Deserialize)]
pub struct RegisterForm {
    name: String,
    email: String,
    password: String,
}

#[post("/register")]
pub async fn register_user(
    state: web::Data<AppState>,
    input: web::Json<RegisterForm>,
) -> impl Responder {
    let hashed_password = hash(input.password.clone(), DEFAULT_COST).unwrap();

    let user = ActiveModel {
        id: NotSet,
        name: Set(input.name.clone()),
        email: Set(input.email.clone()),
        password: Set(hashed_password),
        ..Default::default()
    };

    let user = user.save(&state.db).await.unwrap();

    HttpResponse::Ok().body(
        json!({
            "user": {
                "id": user.id.unwrap(),
                "name": user.name.unwrap(),
            }
        })
        .to_string(),
    )
}

#[derive(serde::Deserialize)]
pub struct LoginRequest {
    email: String,
    password: String,
}

#[post("/login")]
pub async fn login(state: web::Data<AppState>, input: web::Json<LoginRequest>) -> impl Responder {
    let user = User::find()
        .filter(Email.eq(input.email.clone()))
        .one(&state.db)
        .await
        .unwrap()
        .unwrap();
    let correct = verify(input.password.clone(), &user.password).unwrap_or(false);

    if correct {
        let token = encode_jwt(user.id as u32).unwrap();

        HttpResponse::Ok().body(json!({ "token": token }).to_string())
    } else {
        HttpResponse::Ok().body(
            json!({
                "error": "incorrecct redentials"
            })
            .to_string(),
        )
    }
}

pub async fn verify_jwt(
    req: ServiceRequest,
    credentials: BearerAuth,
) -> Result<ServiceRequest, (Error, ServiceRequest)> {

    match decode_jwt(credentials.token()) {
        Ok(decoded) => {
            Ok(req)
        },
        Err(_) => {
            Err((actix_web::error::ErrorForbidden(""), req))
        },
    }


}

pub fn encode_jwt(user_id: u32) -> std::result::Result<String, jsonwebtoken::errors::Error> {
    let claims = Claims {
        sub: user_id,
        exp: (SystemTime::now()
            .duration_since(UNIX_EPOCH)
            .expect("Time is acting wibbily wobbaly")
            .as_secs()
            + 36000) as usize,
    };

    encode::<_>(
        &Header::default(),
        &claims,
        &EncodingKey::from_secret("secret".as_ref()),
    )
}

pub fn decode_jwt(token: &str) -> Result<Claims, jsonwebtoken::errors::Error> {
    let token = decode::<Claims>(
        token,
        &DecodingKey::from_secret("secret".as_ref()),
        &Validation::default(),
    )?;

    Ok(token.claims)
}


pub fn id_from_extractor(extractor: BearerAuth) -> u32 {
    let token = extractor.token();

    decode_jwt(token).unwrap().sub
}