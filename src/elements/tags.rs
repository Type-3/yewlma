use yew::prelude::*;
use yew_property_info::PropertyInfo;
use yewtil::NeqAssign;

use crate::classes::{Size, CssRepr};

pub struct Tags {
    _link: ComponentLink<Self>,
    props: TagsProps,
}

#[derive(Debug, Clone, PartialEq, Properties, PropertyInfo)]
pub struct TagsProps {
    #[prop_or("div")]
    pub tag: &'static str,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub addons: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Tags {
    type Message = ();
    type Properties = TagsProps;

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
        let size = self.props.size.are();
        let addons = self.props.addons.then_some("has-addons");
        html! {
            <@{self.props.tag} class=("tags", addons, size, self.props.class.clone())>{self.props.children.clone()}</@>
        }
    }
}
