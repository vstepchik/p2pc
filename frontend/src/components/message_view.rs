use yew::prelude::*;

pub struct MessageView {
}

pub enum Msg {}

impl Component for MessageView {
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

fn render_view(_cpt: &MessageView) -> Html {
    html! {
        <div class="cpt-msg-view">
            <div class="cpt-msg"><span class="msg-time">{"[2020-04-12 01:19]"}</span><span class="msg-member">{"zun!"}</span><span class="msg-text">{"Hello!"}</span></div>
            <div class="cpt-msg"><span class="msg-time">{"[2020-04-12 01:21]"}</span><span class="msg-member">{"vi?"}</span><span class="msg-text">{"Hello also."}</span></div>
        </div>
    }
}
