use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

pub struct MessageInput {
    link: ComponentLink<Self>,
    value: String,
    input_ref: NodeRef,
    on_enter: Callback<String>,
}

pub enum Msg {
    Nope,
    UpdateValue(String),
    SendMessage,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_enter: Callback<String>,
}

impl Component for MessageInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: String::new(),
            input_ref: NodeRef::default(),
            on_enter: props.on_enter,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SendMessage => {
                self.send_message()
            }
            Msg::UpdateValue(val) => {
                self.value = val;
                false
            }
            _ => false
        };
        true
    }

    fn mounted(&mut self) -> ShouldRender {
        if let Some(input) = self.input_ref.cast::<InputElement>() {
            input.focus().ok();
        }
        false
    }

    fn view(&self) -> Html {
        self.render_view()
    }
}

impl MessageInput {
    fn send_message(&mut self) -> ShouldRender {
        let msg_text: &str = &self.value.trim();
        if msg_text.is_empty() {
            return false;
        } else {
            self.on_enter.emit(msg_text.to_string());
        }

        self.value = String::new();
        true
    }

    fn render_view(&self) -> Html {
        html! {
            <div class="cpt-msg-input">
                <input 
                    type="text"
                    autocomplete="off"
                    ref=self.input_ref.clone()
                    value=&self.value
                    oninput=self.link.callback(|e: InputData| Msg::UpdateValue(e.value))
                    onkeypress=self.link.callback(|e: KeyboardEvent| {
                        if e.key() == "Enter" { Msg::SendMessage } else { Msg::Nope }
                    })
                />
                <button onclick=self.link.callback(|_| Msg::SendMessage)>{ "Send!" }</button>
            </div>
        }
    }
}
