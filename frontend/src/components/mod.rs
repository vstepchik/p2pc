use yew::prelude::ShouldRender;

pub mod chat;
pub mod member_list;
pub mod message_input;
pub mod message_view;

pub trait NeqAssign {
    fn neq_assign(&mut self, new: Self) -> ShouldRender;
}

impl<T: PartialEq> NeqAssign for T {
    fn neq_assign(&mut self, new: T) -> ShouldRender {
        if self != &new {
            *self = new;
            true
        } else {
            false
        }
    }
}
