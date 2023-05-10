use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};
use uuid::Uuid;

#[derive(Debug, Serialize, Deserialize, Clone, PartialEq, Selectable, Queryable, Insertable)]
pub struct User {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
    pub password: String,
}

#[derive(Debug, Serialize)]
pub struct Me {
    pub id: Uuid,
    pub nickname: String,
    pub email: String,
}
