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

use crate::constants;
use crate::errors::ServiceError;
use axum::http::StatusCode;
use bcrypt::{hash, verify, DEFAULT_COST};
use std::env;

pub fn hash_password(plain: &str) -> Result<String, ServiceError> {
    let hashing_cost: u32 = match env::var("HASH_ROUNDS") {
        Ok(cost) => cost.parse().unwrap_or(DEFAULT_COST),
        _ => DEFAULT_COST,
    };
    hash(plain, hashing_cost).map_err(|_| {
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
        )
    })
}
// VERIFY PASSWORD
pub fn compare_password(plain: &str, hash: &str) -> Result<bool, ServiceError> {
    verify(plain, hash).map_err(|_| {
        ServiceError::new(
            StatusCode::INTERNAL_SERVER_ERROR,
            constants::MESSAGE_PROCESS_TOKEN_ERROR.to_string(),
        )
    })
}
