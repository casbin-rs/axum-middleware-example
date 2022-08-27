// Copyright 2022 The casbin Authors. All Rights Reserved.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//      http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

use crate::model::{
    db::Pool,
    user::User,
    user_token::{UserToken, KEY},
};
use axum::extract::Extension;
use jsonwebtoken::{DecodingKey, TokenData, Validation};

// DECODE THE GOT TOKEN
pub fn decode_token(token: String) -> jsonwebtoken::errors::Result<TokenData<UserToken>> {
    jsonwebtoken::decode::<UserToken>(
        &token,
        &DecodingKey::from_secret(&KEY),
        &Validation::default(),
    )
}

// VALIDATE TOKEN
pub fn validate_token(
    token_data: &TokenData<UserToken>,
    Extension(pool): &Extension<Pool>,
) -> Result<String, String> {
    if User::is_valid_login_session(&token_data.claims, &pool.get().unwrap()) {
        Ok(token_data.claims.user_name.to_string())
    } else {
        Err("Invalid token".to_string())
    }
}
