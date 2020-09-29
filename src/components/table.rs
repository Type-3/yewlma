use yew::prelude::*;
use yewtil::{Pure, PureComponent};

pub type Table = Pure<PureTable>;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct PureTable {
    #[prop_or_default]
    pub striped: bool,
    #[prop_or_default]
    pub bordered: bool,
    #[prop_or_default]
    pub fullwidth: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children,
}

impl PureComponent for PureTable {
    fn render(&self) -> Html {
        let striped = self.striped.then_some("is-striped");
        let fullwidth = self.fullwidth.then_some("is-fullwidth");
        let bordered = self.bordered.then_some("is-bordered");
        html! {
            <table class=("table", striped, bordered, fullwidth, &self.class)>
              { self.children.clone() }
            </table>
        }
    }
}
