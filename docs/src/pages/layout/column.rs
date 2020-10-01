use crate::components::{DemoContainer, PropsTable};
use yew::prelude::*;
use yewlma::layout::ColumnProps;
use yewlma::prelude::*;

pub struct ColumnPage {
    _link: ComponentLink<Self>,
}

pub enum Msg {}

impl Component for ColumnPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ColumnPage { _link }
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
              <Columns>
              <Column>
                 <h1 class="title">{"Properties"}</h1>
                 <DemoContainer>
                     <PropsTable<ColumnProps> />
                 </DemoContainer>
               </Column>
              </Columns>
            </Container>
        }
    }
}
