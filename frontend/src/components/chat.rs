use std::rc::Rc;
use std::cell::RefCell;
use chrono::prelude::*;
use yew::prelude::*;
use crate::components::{
    member_list::MemberList,
    message_view::MessageView,
    message_input::MessageInput,
    nickname_input::NicknameInput,
};
use crate::model::{Member, Message};

pub struct Chat {
    link: ComponentLink<Self>,
    local_member: Option<Rc<Member>>,
    members: Rc<Vec<Rc<Member>>>,
    messages: RefCell<Vec<Message>>,
}

pub enum Msg {
    SetLocalMember(String),
    CreateMessage(String),
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let members = Rc::new(vec![
            Rc::new(Member::new("vi?")),
            Rc::new(Member::new("zun!")),
        ]);
        let messages = RefCell::new(vec![
            Message::new(&members[1], &Utc::now(), "Hello!!"),
            Message::new(&members[0], &Utc::now(), "Hello also."),
        ]);

        Self { link, local_member: None, members, messages }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetLocalMember(nick) => {
                let local_member = Rc::new(Member::new(&nick));
                self.local_member = Some(Rc::clone(&local_member));
            }
            Msg::CreateMessage(mut text) => {
                if let Some(member) = &self.local_member {
                    if text == "/shrug" {
                        text = r"¯\_(ツ)_/¯".to_string();
                    }
                    let msg = Message { from: Rc::clone(&member), sent_at: Utc::now(), text: Rc::new(text) };
                    self.messages.borrow_mut().push(msg);
                }
            }
        };
        true
    }

    fn view(&self) -> Html {
        self.render_view()
    }
}

impl Chat {
    fn render_view(&self) -> Html {
        html! {
            <div class="container-global flex-row">
                {
                    if self.local_member.is_some() {
                        self.render_messages()
                    } else {
                        self.render_intro()
                    }
                }
                <div class="container-member-list"><MemberList members=&self.members></MemberList></div>
            </div>
        }
    }
    
    fn render_messages(&self) -> Html {
        html! {
            <div class="container-chat">
                <div class="container-msg-view"><MessageView messages=&self.messages /></div>
                <div class="container-msg-input"><MessageInput on_enter=self.link.callback(|msg| Msg::CreateMessage(msg)) /></div>
            </div>
        }
    }

    fn render_intro(&self) -> Html {
        html! {
            <div class="container-intro">
                <NicknameInput on_set=self.link.callback(|nick| Msg::SetLocalMember(nick)) />
            </div>
        }
    }
}
