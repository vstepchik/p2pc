use std::rc::Rc;
use yew::prelude::*;
use crate::model::Member;
use crate::components::NeqAssign;

pub struct MemberList {
    props: Props,
}

pub enum Msg {}

#[derive(Properties, PartialEq, Clone)]
pub struct Props {
    pub members: Rc<Vec<Rc<Member>>>,
}

impl Component for MemberList {
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

fn render_view(cpt: &MemberList) -> Html {
    let render_member = |m: &Rc<Member>| html! {
        <div class="cpt-member"> { &m.name } </div>
    };

    html! {
        <div class="cpt-member-list">
            { for cpt.props.members.iter().map(render_member) }
        </div>
    }
}
