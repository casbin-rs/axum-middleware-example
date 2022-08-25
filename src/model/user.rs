#![allow(clippy::extra_unused_lifetimes)]

use crate::{
    constants,
    schema::users::{self, dsl::*},
};

use axum_macros::FromRequest;
use diesel::prelude::*;
use diesel::QueryDsl;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Queryable)]
// #[derive(Identifiable, Queryable, PartialEq, Debug)]
#[diesel(table_name = users)]
pub struct User {
    pub id: i32,
    pub username: String,
    pub email: String,
    pub password: String,
    pub role: String,
    pub login_session: String,
}

#[derive(Insertable, Serialize, Deserialize, AsChangeset, Clone, FromRequest)]
#[table_name = "users"]
pub struct AddUser {
    pub username: String,
    pub email: String,
    pub password: String,
    #[serde(default = "default_role")]
    pub role: String,
    pub login_session: String,
}

fn default_role() -> String {
    "patient".to_owned()
}

#[derive(Serialize, Deserialize)]
pub struct LoginForm {
    pub email: String,
    pub password: String,
}

#[derive(Insertable)]
#[table_name = "users"]
pub struct LoginInfo {
    // WILL GET UNIQUE USERNAME BY EMAIL
    pub username: String,
    pub role: String,
    pub login_session: String,
}

impl User {
    pub fn register() {}

    pub fn update_user() {}

    pub fn get_user() {}

    pub fn get_all_user() {}

    pub fn signin() {}

    pub fn delete_user() {}

    pub fn update_login_session_to_db() {}

    pub fn is_valid_login_session() {}

    pub fn get_user_by_email() {}

    pub fn generate_login_session() {}
}
