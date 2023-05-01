use crate::schema::*;
use diesel::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize, Queryable, Identifiable)]
pub struct Crypto {
    pub id: u64,
    pub name: String,
    pub symbol: String,
}
