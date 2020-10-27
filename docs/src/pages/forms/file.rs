use crate::components::{DemoContainer, PropertyInformation};
use yew::prelude::*;
use yewlma::forms::{FileField, FileFieldProps};
use yewlma::prelude::*;

pub struct FilePage;

impl Component for FilePage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        FilePage
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
                    <FileField />
                    </div>
                    </Column>
                    </Columns>
                    </DemoContainer>
                 </Column>
              </Columns>
              <PropertyInformation<FileFieldProps> />
            </Container>
        }
    }
}
