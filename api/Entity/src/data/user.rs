use crate::user::{ActiveModel, Model};
use crate::db::conn::{get_conn};
use argon2::{self, Config};
use sea_orm::{ActiveModelTrait, DbErr};
use serde::{Serialize, Deserialize};
use sea_orm::ActiveValue::Set;

#[derive(Serialize, Deserialize, Debug, Default)]
#[serde(default)]
pub struct SignInRequest {
    pub username: String,
    pub email:    String,
    pub password: String,
}

#[derive(Serialize, Deserialize, Debug)]
// #[serde(default)]
pub struct SignInResponse {
    pub msg: String,
}

pub async fn sign_up(username: &String, email: &String, password: &String) -> () {
    let conn = get_conn().await.to_owned();

    let salt = b"randomsalt";
    let config = Config::default();
    let pwd = password.as_bytes();
    let hashed_password = argon2::hash_encoded(pwd, salt, &config).unwrap();

    let user = ActiveModel {
        username: Set(username.to_owned()),
        email: Set(email.to_owned()),
        password: Set(hashed_password.to_owned()),
        ..Default::default()
    }
    .save(&conn)
    .await
    .expect("Could Not Sign Up");

}