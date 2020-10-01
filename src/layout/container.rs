use yew::prelude::*;
use yew_property_info::PropertyInfo;
use yewtil::{Pure, PureComponent};

use crate::classes::{BreakPoint, Color, CssRepr};

pub type Container = Pure<ContainerProps>;

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct ContainerProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or_default]
    pub bg: Option<Color>,
    #[prop_or_default]
    pub breakpoint: Option<BreakPoint>,
    #[prop_or_default]
    pub fluid: bool,
    #[prop_or("div")]
    pub tag: &'static str,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub style: String,
    #[prop_or_default]
    pub children: Children,
}

impl PureComponent for ContainerProps {
    fn render(&self) -> Html {
        let breakpoint = self.breakpoint.is();
        let fluid = self.fluid.then_some("is-fluid");
        let bg = &self.bg.has_bg();
        html! {
            <@{self.tag}
                id?=self.id.as_ref()
                style=&self.style
                class=("container", fluid, breakpoint, bg, &self.class)>
                { self.children.clone() }
            </@>
        }
    }
}
