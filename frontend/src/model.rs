use std::rc::Rc;

#[derive(PartialEq, Clone)]
pub struct Member {
    pub name: Rc<String>,
}
