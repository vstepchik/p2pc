use yew::prelude::*;

pub struct MemberList {
}

pub enum Msg {}

impl Component for MemberList {
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

fn render_view(_cpt: &MemberList) -> Html {
    html! {
        <div class="cpt-member-list">
            <div class="cpt-member">{ "vi?" }</div>
            <div class="cpt-member">{ "zun!" }</div>
        </div>
    }
}
