use actix_web::{web, Responder, HttpResponse, post};

use entity::data::user::{SignInRequest,sign_up, SignInResponse, User};

#[post("/sign_up")]
pub async fn create(form: web::Form<SignInRequest>) -> impl Responder {
    let model = sign_up(&form.username, &form.email, &form.password).await;

    let user = User { 
        id: model.id , 
        username: model.username, 
        email: model.email,
        password: model.password, 
        created_at: model.created_at, 
        updated_at: model.updated_at, 
    };

    let response = SignInResponse {
        user: user
    };

    HttpResponse::Ok().json(response)
}
