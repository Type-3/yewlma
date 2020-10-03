use derive_more::Display;
use yew::prelude::*;
use yewtil::NeqAssign;

#[derive(Debug, Display, PartialEq, Clone, Copy)]
pub enum BulmaCdn {
    #[display(fmt = "https://cdn.jsdelivr.net/npm/bulma@0.9.0/css/bulma.min.css")]
    JsDelvr,
    #[display(fmt = "https://cdnjs.cloudflare.com/ajax/libs/bulma/0.9.0/css/bulma.min.css")]
    CdnJs,
}

pub struct Cdn(Props);

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Props {
    pub cdn: BulmaCdn,
}

impl Component for Cdn {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        Cdn(props)
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.0.neq_assign(props)
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <link rel="stylesheet" href={self.0.cdn.to_string()} />
        }
    }
}
