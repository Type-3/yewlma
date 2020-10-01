use yew::prelude::*;
use yew_property_info::PropertyInfo;
use yewtil::{Pure, PureComponent};

pub type Buttons = Pure<ButtonsProps>;

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct ButtonsProps {
    #[prop_or_default]
    pub addons: bool,
    #[prop_or_default]
    pub center: bool,
    #[prop_or_default]
    pub right: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

impl PureComponent for ButtonsProps {
    fn render(&self) -> Html {
        let addons = self.addons.then_some("has-addons");
        let right = self.right.then_some("is-right");
        let center = self.center.then_some("is-centered");
        html! {
            <div class=("buttons", addons, right, center, &self.class)>
              { self.children.clone() }
            </div>
        }
    }
}
