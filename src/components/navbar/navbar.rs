use crate::prelude::*;
use yew::prelude::*;
use yewtil::{Pure, PureComponent};

pub type NavBar = Pure<PureNavBar>;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct PureNavBar {
    #[prop_or("nav")]
    pub tag: &'static str,
    #[prop_or_default]
    pub shadow: bool,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub fixed_top: bool,
    #[prop_or_default]
    pub fixed_bottom: bool,
    #[prop_or_default]
    pub children: Children,
}

impl PureComponent for PureNavBar {
    fn render(&self) -> Html {
        let shadow = self.shadow.then_some("has-shadow");
        let color = self.color.is();
        let fixed_top = self.fixed_top.then_some("is-fixed-top");
        let fixed_bottom = self.fixed_bottom.then_some("is-fixed-bottom");
        html! {
            <@{self.tag} class=("navbar", color, fixed_top, fixed_bottom, shadow)>
              <Container>
                { self.children.clone() }
              </Container>
            </@>
        }
    }
}
