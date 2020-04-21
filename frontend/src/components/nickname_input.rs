use web_sys::HtmlInputElement as InputElement;
use yew::prelude::*;

pub struct NicknameInput {
    link: ComponentLink<Self>,
    value: String,
    input_ref: NodeRef,
    on_set: Callback<String>,
}

pub enum Msg {
    Nope,
    UpdateValue(String),
    SetNickname,
}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub on_set: Callback<String>,
}

impl Component for NicknameInput {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: String::new(),
            input_ref: NodeRef::default(),
            on_set: props.on_set,
        }
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SetNickname => {
                self.set_nickname()
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

impl NicknameInput {
    fn set_nickname(&mut self) -> ShouldRender {
        let msg_text: &String = &self.value;
        if msg_text.trim().is_empty() {
            return false;
        } else {
            self.on_set.emit(self.value.trim().to_string());
        }
        true
    }

    fn render_view(&self) -> Html {
        html! {
            <div class="cpt-nick-input">
                <div>{ "Introduce yourself:" }</div>
                <div class="cpt-nick-input-controls">
                    <input 
                        type="text"
                        autocomplete="off"
                        ref=self.input_ref.clone()
                        value=&self.value
                        oninput=self.link.callback(|e: InputData| Msg::UpdateValue(e.value))
                        onkeypress=self.link.callback(|e: KeyboardEvent| {
                            if e.key() == "Enter" { Msg::SetNickname } else { Msg::Nope }
                        })
                    />
                    <button onclick=self.link.callback(|_| Msg::SetNickname)>{ "Fight!" }</button>
                </div>
            </div>
        }
    }
}
