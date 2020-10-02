use yew::prelude::*;
use yewlma::prelude::*;
use yewlma::components::TabsProps;
use crate::components::{DemoContainer, PropertyInformation};

pub struct TabsPage {
    link: ComponentLink<Self>,
    demo_boxed: bool,
    demo_toggle: bool,
    demo_round: bool
}

pub enum Msg {
    ToggleBoxed(bool),
    ToggleToggle(bool),
    ToggleRound(bool)
}

impl Component for TabsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            demo_boxed: false,
            demo_toggle: false,
            demo_round: false
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleBoxed(value) => self.demo_boxed = value,
            Msg::ToggleToggle(value) => self.demo_toggle = value,
            Msg::ToggleRound(value) => self.demo_round = value
        }
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
                      <Columns>
                        <Column><CheckBoxField value=self.demo_boxed label="boxed" on_toggle=self.link.callback(Msg::ToggleBoxed) /></Column>
                        <Column><CheckBoxField value=self.demo_toggle label="toggle" on_toggle=self.link.callback(Msg::ToggleToggle) /></Column>
                        <Column><CheckBoxField value=self.demo_round label="toggle round" on_toggle=self.link.callback(Msg::ToggleRound) /></Column>
                      </Columns>
                    </Column>
                    <Column style="text-align:center">
                    <div class="box">
                    <Tabs toggle=self.demo_toggle
                               toggle_round=self.demo_round
                               boxed=self.demo_boxed
                               titles=vec!["One".into(), "Two".into(), "Three".into(), "Four".into()]>
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
              <PropertyInformation<TabsProps> />
            </Container>
        }
    }
}
