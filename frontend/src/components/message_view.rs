use std::cell::RefCell;
use yew::prelude::*;
use crate::model::Message;
use crate::components::NeqAssign;

pub struct MessageView {
    props: Props,
}

pub enum Msg {}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub messages: RefCell<Vec<Message>>,
}

impl Component for MessageView {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Self { props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, _msg: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        render_view(self)
    }
}

fn render_view(cpt: &MessageView) -> Html {
    fn render_msg(m: &Message) -> Html {
        html! {
            <div class="cpt-msg">
                <span class="msg-time">{ format!("[{}]", &m.sent_at.format("%y-%m-%d %T")) }</span>
                <span class="msg-member">{ &m.from.name }</span>
                <span class="msg-text">{ &m.text }</span>
            </div>
        }
    }

    html! {
        <div class="cpt-msg-view">
            { for cpt.props.messages.borrow().iter().map(render_msg) }
        </div>
    }
}
