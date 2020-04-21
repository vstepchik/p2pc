use std::rc::Rc;
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
    messages: Rc<Vec<Message>>,
}

pub enum Msg {
    SetLocalMember(String),
}

impl Component for Chat {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        let members = Rc::new(vec![
            Rc::new(Member::new("vi?")),
            Rc::new(Member::new("zun!")),
        ]);
        let messages = Rc::new(vec![
            Message { from: Rc::clone(&members[1]), sent_at: Utc::now(), text: Rc::new("Hello!!".to_string()) },
            Message { from: Rc::clone(&members[0]), sent_at: Utc::now(), text: Rc::new("Hello also.".to_string()) },
        ]);

        Self { link, local_member: None, members, messages }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetLocalMember(nick) => {
                let local_member = Rc::new(Member::new(&nick));
                self.local_member = Some(Rc::clone(&local_member));
                false
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
                <div class="container-msg-input"><MessageInput /></div>
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
