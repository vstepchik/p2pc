use crate::protocol::model::Session;
use serde::{Deserialize, Serialize};

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Login(pub Session);

#[derive(Debug, PartialEq, Deserialize, Serialize)]
pub struct Logout;
