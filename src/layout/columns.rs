use yew::prelude::*;
use yew_property_info::PropertyInfo;
use yewtil::{Pure, PureComponent};

use crate::classes::{BreakPoint, CssRepr};

pub type Columns = Pure<ColumnsProps>;

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct ColumnsProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or("div")]
    pub tag: &'static str,
    #[prop_or_default]
    pub breakpoint: Option<BreakPoint>,
    #[prop_or_default]
    pub gapless: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

impl PureComponent for ColumnsProps {
    fn render(&self) -> Html {
        let breakpoint = self.breakpoint.is();
        let gapless = self.gapless.then_some("is-gapless");
        html! {
            <@{self.tag}
                id?=self.id.as_ref()
                class=("columns", breakpoint, gapless, &self.class)>
                { self.children.clone() }
            </@>
        }
    }
}
