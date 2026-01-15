// Users
use serde::{Deserialize, Serialize};
use utoipa::ToSchema;

#[derive(Serialize, Deserialize, ToSchema)]
#[schema(example = json!({"id": 1, "username": "Test Item"}))]

    pub struct User {
        id: u64,
        username: String,
    }