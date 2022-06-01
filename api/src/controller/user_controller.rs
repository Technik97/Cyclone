use actix_web::{web, Responder, HttpResponse, post};

use entity::data::user::{SignInRequest, sign_up, SignInResponse};
use entity::data::error::{ErrorResponse};


#[post("/sign_up")]
pub async fn create(form: web::Form<SignInRequest>) -> impl Responder {
    let response = sign_up(&form.username, &form.email, &form.password).await;

    match response {
        model => {
            // let res = SignInResponse {
            //     msg: "Signed Up Successfully".to_string(),
            // };

            HttpResponse::Ok().finish()  //.content_type("application/json").json(res)
        },
        db_err => {
            // let resp = ErrorResponse{
            //     err_msg: "Cannot Resgiter".to_owned(),
            // };

            HttpResponse::BadRequest().finish()    //.content_type("application/json").json(resp)
        },
    }
}
