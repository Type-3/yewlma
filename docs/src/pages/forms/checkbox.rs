use yew::prelude::*;
use yewlma::prelude::*;
use yewlma::forms::CheckBoxFieldProps;
use crate::components::{DemoContainer, PropsTable};

pub struct CheckBoxPage;

impl Component for CheckBoxPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        CheckBoxPage
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
                    <DemoContainer>
                    <Columns class="is-vcentered">
                    <Column style="text-align:center">
                    <div class="box">
                    <CheckBoxField label="CheckBox" />
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
                     <PropsTable<CheckBoxFieldProps> />
                 </DemoContainer>
               </Column>
              </Columns>
            </Container>
        }
    }
}
