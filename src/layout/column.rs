use yew::prelude::*;
use yew_property_info::PropertyInfo;
use yewtil::{Pure, PureComponent};

use crate::classes::{ColSize, CssRepr, Narrow};

pub type Column = Pure<ColumnProps>;

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct ColumnProps {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or("div")]
    pub tag: &'static str,
    #[prop_or_default]
    pub style: Option<String>,
    #[prop_or_default]
    pub size: Option<ColSize>,
    #[prop_or_default]
    pub offset: Option<ColSize>,
    #[prop_or_default]
    pub narrow: Option<Narrow>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

impl PureComponent for ColumnProps {
    fn render(&self) -> Html {
        let size = self.size.is();
        let narrow = self.narrow.is();
        let offset = self.offset.is_offset();
        html! {
            <@{self.tag}
                id?=self.id.as_ref()
                style?=self.style.as_ref()
                class=("column", size, offset, narrow, &self.class)>
                { self.children.clone() }
            </@>
        }
    }
}
