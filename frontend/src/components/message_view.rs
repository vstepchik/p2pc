use std::cell::RefCell;
use js_sys;
use chrono::{DateTime, Local, FixedOffset, NaiveDateTime};
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
        self.render_view()
    }
}

impl MessageView {
    fn render_view(&self) -> Html {
        fn from_utc_datetime(utc: &NaiveDateTime) -> DateTime<Local> {
            // Get the offset from the js runtime
            let offset = FixedOffset::west((js_sys::Date::new_0().get_timezone_offset() as i32) * 60);
            DateTime::from_utc(*utc, offset)
        }

        fn render_msg(m: &Message) -> Html {
            html! {
                <div class="cpt-msg">
                    // until resolved: https://github.com/chronotope/chrono/issues/411
                    // <span class="msg-time">{ format!("[{}]", &m.sent_at.with_timezone(&Local).format("%y-%m-%d\u{00A0}%T %:z")) }</span>
                    <span class="msg-time">{ format!("[{}]", from_utc_datetime(&m.sent_at.naive_utc()).format("%y-%m-%d\u{00A0}%T")) }</span>
                    <span class="msg-member">{ &m.from.name }</span>
                    <span class="msg-text">{ &m.text }</span> // todo: fix wrapping of super-long words
                </div>
            }
        }
    
        html! {
            <div class="cpt-msg-view">
                { for self.props.messages.borrow().iter().map(render_msg) }
            </div>
        }
    }    
}
