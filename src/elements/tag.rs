use crate::prelude::*;
use yew::prelude::*;
use yew_property_info::PropertyInfo;
use yewtil::NeqAssign;

pub struct Tag {
    _link: ComponentLink<Self>,
    props: TagProps,
}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct TagProps {
    #[prop_or("span")]
    pub tag: &'static str,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub light: bool,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Tag {
    type Message = ();
    type Properties = TagProps;

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
        let color = self.props.color.is();
        let size = self.props.size.is();
        let light = self.props.light.then_some("is-light");
        let round = self.props.rounded.then_some("is-rounded");
        html! {
            <@{self.props.tag} class=("tag", size, color, light, round, &self.props.class)>
                {self.props.children.clone()}
            </@>
        }
    }
}
