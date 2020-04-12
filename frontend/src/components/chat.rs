use std::rc::Rc;
use yew::prelude::*;
use crate::components::{
    member_list::MemberList,
    message_view::MessageView,
    message_input::MessageInput,
};
use crate::model::Member;

pub struct Chat {
    members: Rc<Vec<Member>>,
}

pub enum Msg {}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        let members = Rc::new(vec![
            Member { name: Rc::new("vi?".to_string()) },
            Member { name: Rc::new("zun!".to_string()) },
            Member { name: Rc::new("<somebody>".to_string()) },
        ]);

        Self { members }
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        render_view(self)
    }
}

fn render_view(cpt: &Chat) -> Html {
    html! {
        <div class="container-global flex-row">
            <div class="container-chat">
                <div class="container-msg-view"><MessageView /></div>
                <div class="container-msg-input"><MessageInput /></div>
            </div>
            <div class="container-member-list"><MemberList members=&cpt.members></MemberList></div>
        </div>
    }
}
