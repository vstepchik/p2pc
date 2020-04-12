use std::rc::Rc;
use chrono::{DateTime, Utc};

#[derive(PartialEq, Clone)]
pub struct Member {
    pub name: Rc<String>,
}

#[derive(PartialEq, Clone)]
pub struct Message {
    pub from: Rc<Member>,
    pub sent_at: DateTime<Utc>,
    pub text: Rc<String>,
}
