use crate::components::{DemoContainer, PropsTable};
use yew::prelude::*;
use yewlma::components::TabsProps;
use yewlma::prelude::*;

pub struct TabsPage {
    _link: ComponentLink<Self>,
}

pub enum Msg {

}

impl Component for TabsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self {
            _link,
        }
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
                    <Column style="text-align:center">
                    <div class="box">
                    <Tabs titles=vec!["One".into(), "Two".into(), "Three".into(), "Four".into()]>
                        <div>{"Tab One"}</div>
                        <div>{"Tab Two"}</div>
                        <div>{"Tab Three"}</div>
                        <div>{"Tab Four"}</div>
                    </Tabs>
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
                     <PropsTable<TabsProps> />
                 </DemoContainer>
               </Column>
              </Columns>
            </Container>
        }
    }
}
