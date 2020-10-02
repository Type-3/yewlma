use yew::prelude::*;
use yewtil::NeqAssign;
use yew_property_info::PropertyInfo;

pub struct Footer {
    _link: ComponentLink<Self>,
    props: FooterProps
}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct FooterProps {
    #[prop_or("footer")]
    pub tag: &'static str,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children
}

impl Component for Footer {
    type Properties = FooterProps;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props, _link }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <@{self.props.tag} class=("footer", &self.props.class)>
                {self.props.children.clone()}
            </@>
        }
    }
}
