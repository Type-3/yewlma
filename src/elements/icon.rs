use yew::prelude::*;
use yew_property_info::PropertyInfo;
use yewtil::{Pure, PureComponent};

use crate::prelude::*;

pub type Icon = Pure<IconProps>;

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct IconProps {
    pub name: String,
    #[prop_or("span")]
    pub tag: &'static str,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub class: Option<String>,
}

impl PureComponent for IconProps {
    fn render(&self) -> Html {
        let size = self.size.is();
        let color = self.color.has_text();
        html! {
            <@{self.tag} class=("icon", size, color, &self.class)>
              <i class=&self.name></i>
            </@>
        }
    }
}
