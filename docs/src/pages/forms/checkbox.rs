use crate::components::{DemoContainer, PropertyInformation};
use yew::prelude::*;
use yewlma::forms::CheckBoxFieldProps;
use yewlma::prelude::*;

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
              <PropertyInformation<CheckBoxFieldProps> />
            </Container>
        }
    }
}
