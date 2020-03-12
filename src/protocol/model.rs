use serde::{Deserialize, Serialize};
use std::convert::From;

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Session {
    pub token: String,
}

impl Session {
    pub fn new<T>(token: T) -> Self
    where
        String: From<T>,
    {
        Self {
            token: token.into(),
        }
    }
}
