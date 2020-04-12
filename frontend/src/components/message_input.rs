use yew::prelude::*;

pub struct MessageInput {
}

pub enum Msg {}

impl Component for MessageInput {
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

fn render_view(_cpt: &MessageInput) -> Html {
    html! {
        <div class="cpt-msg-input">
            <input name="Message" type="text" autocomplete="off" />
            <button>{ "Send!" }</button>
        </div>
    }
}
