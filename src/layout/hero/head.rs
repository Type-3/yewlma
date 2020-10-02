use yew::prelude::*;
use yewtil::NeqAssign;
use yew_property_info::PropertyInfo;

pub struct HeroHead {
    _link: ComponentLink<Self>,
    props: HeroHeadProps
}

pub enum Msg {

}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct HeroHeadProps {
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children
}

impl Component for HeroHead {
    type Message = Msg;
    type Properties = HeroHeadProps;

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
            <div class=("hero-head", &self.props.class)>
                {self.props.children.clone()}
            </div>
        }
    }
}
