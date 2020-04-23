use std::rc::Rc;
use chrono::{DateTime, Utc};

#[derive(PartialEq, Clone)]
pub struct Member {
    pub name: Rc<String>,
}

impl Member {
    pub fn new(name: &str) -> Self {
        Member { name: Rc::new(name.to_string()) }
    }
}

#[derive(PartialEq, Clone)]
pub struct Message {
    pub from: Rc<Member>,
    pub sent_at: DateTime<Utc>,
    pub text: Rc<String>,
}

impl Message {
    pub fn new(from: &Rc<Member>, sent_at: &DateTime<Utc>, text: &str) -> Self {
        Self { from: Rc::clone(&from), sent_at: *sent_at, text: Rc::new(text.to_string()) }
    }
}
