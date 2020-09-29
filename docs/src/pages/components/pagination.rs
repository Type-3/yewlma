use yewlma::prelude::*;
use yew::prelude::*;
use yewlma::components::PaginationProps;
use crate::components::{DemoContainer, PropsTable};

pub struct PaginationPage;

pub enum Msg {}

impl Component for PaginationPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        PaginationPage
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        true
    }

    fn view(&self) -> Html {
        html! {
            <Container>
              <Columns>
                 <Column>
                    <DemoContainer>
                    <Columns class="is-vcentered">
                    <Column>
                    </Column>
                    <Column style="text-align:center">
                    <div class="box">
                    <Pagination total=200 per_page=10 page=5 />
                    </div>
                    </Column>
                    </Columns>
                    </DemoContainer>
                 </Column>
              </Columns>
              <Columns>
              <Column>
                 <h1 class="title">{"Properties"}</h1>
                 <DemoContainer>
                     <PropsTable<PaginationProps> />
                 </DemoContainer>
               </Column>
              </Columns>
            </Container>
        }
    }
}
