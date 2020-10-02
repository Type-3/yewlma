use yew::prelude::*;
use yewlma::layout::FooterProps;
use yewlma::prelude::*;
use crate::components::PropertyInformation;

pub struct FooterPage {
    _link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for FooterPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        FooterPage { _link }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container>
              <PropertyInformation<FooterProps> />
            </Container>
        }
    }
}
