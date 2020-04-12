use yew::prelude::*;
use crate::components::{
    member_list::MemberList,
    message_view::MessageView,
    message_input::MessageInput,
};

pub struct Chat {
}

pub enum Msg {}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self {}
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        render_view(self)
    }
}

fn render_view(_cpt: &Chat) -> Html {
    html! {
        <div class="container-global flex-row">
            <div class="container-chat">
                <div class="container-msg-view"><MessageView /></div>
                <div class="container-msg-input"><MessageInput /></div>
            </div>
            <div class="container-member-list"><MemberList /></div>
        </div>
    }
}
