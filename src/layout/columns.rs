use yew::prelude::*;
use yewtil::{Pure, PureComponent};

use crate::classes::{BreakPoint, CssRepr};

pub type Columns = Pure<PureColumns>;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct PureColumns {
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

impl PureComponent for PureColumns {
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
