use yew::prelude::*;
use yewlma::prelude::*;
use yewlma::layout::ColumnsProps;
use crate::components::PropertyInformation;

pub struct ColumnsPage {
    _link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for ColumnsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { _link }
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
              <PropertyInformation<ColumnsProps> />
            </Container>
        }
    }
}
