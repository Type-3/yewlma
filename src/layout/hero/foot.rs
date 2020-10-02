use yew::prelude::*;
use yewtil::NeqAssign;
use yew_property_info::PropertyInfo;

pub struct HeroFoot {
    _link: ComponentLink<Self>,
    props: HeroFootProps
}

pub enum Msg {

}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct HeroFootProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children
}

impl Component for HeroFoot {
    type Message = Msg;
    type Properties = HeroFootProps;

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { _link, props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class=("hero-foot", &self.props.class)>
                {self.props.children.clone()}
            </div>
        }
    }
}
