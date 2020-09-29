use crate::prelude::*;
use yew::prelude::*;
use yewtil::{Pure, PureComponent};

pub type Card = Pure<PureCard>;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct PureCard {
    #[prop_or("div")]
    pub tag: &'static str,
    #[prop_or_default]
    pub bg: Option<Color>,
    #[prop_or_default]
    pub fg: Option<Color>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

impl PureComponent for PureCard {
    fn render(&self) -> Html {
        let bg = self.bg.has_bg();
        let fg = self.fg.has_text();
        html! {
            <@{self.tag} class=("card", bg, fg, &self.class)>
              { self.children.clone() }
            </@>
        }
    }
}
