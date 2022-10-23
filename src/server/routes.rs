use actix_web::{
    web::{self, Json},
    App, HttpResponse, HttpServer, Responder,
};
use sqlx::PgPool;

use crate::domain::users::{
    services as users_service,
    types::{CreateUserInput, User},
};

#[derive(serde::Serialize)]
pub struct HealthCheckResponse {
    pub status: String,
}

pub async fn health_check() -> Json<HealthCheckResponse> {
    let response = HealthCheckResponse {
        status: String::from("Ok"),
    };

    return Json(response);
}

#[derive(serde::Deserialize, Debug, Clone)]
pub struct SignUpRequestBody {
    pub email: String,
    pub password: String,
    pub username: String,
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct SignUpResponse {
    pub token: String,
}

pub async fn sign_up(sign_up_input: Json<SignUpRequestBody>) -> Json<SignUpResponse> {
    println!("Sign up request body: {:?}", sign_up_input);

    let create_user_input = CreateUserInput {
        email: sign_up_input.email.clone(),
        password: sign_up_input.password.clone(),
        username: sign_up_input.username.clone(),
    };

    let user = users_service::create(create_user_input).await.unwrap();
    println!("Created user: {:?}", user);

    let response = SignUpResponse {
        token: String::from("computed token"),
    };

    return Json(response);
}

#[derive(serde::Serialize, Debug, Clone)]
pub struct GetUsersResponse {
    pub users: Vec<User>,
}

#[get("/users")]
pub async fn get_users(pool: web::Data<PgPool>) -> Json<String> {
    println!("Getting users");

    let rows = sqlx::query!(
        "SELECT * FROM users"
    ).fetch_all(pool.get_ref()).await.expect("Error");

    return Json("Ok".to_string());
}
