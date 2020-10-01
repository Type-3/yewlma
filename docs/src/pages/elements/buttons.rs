use crate::components::{DemoContainer, PropsTable};
use yew::prelude::*;
use yewlma::elements::ButtonsProps;
use yewlma::prelude::*;

pub struct ButtonsPage {
    link: ComponentLink<Self>,
    demo_addons: bool,
    demo_right: bool,
    demo_center: bool,
}

pub enum Msg {
    ToggleAddons(bool),
    ToggleRight(bool),
    ToggleCenter(bool),
}

impl Component for ButtonsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ButtonsPage {
            link,
            demo_addons: false,
            demo_right: false,
            demo_center: false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleAddons(value) => self.demo_addons = value,
            Msg::ToggleRight(value) => self.demo_right = value,
            Msg::ToggleCenter(value) => self.demo_center = value,
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
                    <Columns class="py-3 px-2">
                        <Column><CheckBoxField value=self.demo_addons label="addons" on_toggle=self.link.callback(Msg::ToggleAddons)/></Column>
                        <Column><CheckBoxField value=self.demo_right label="right" on_toggle=self.link.callback(Msg::ToggleRight)/></Column>
                        <Column><CheckBoxField value=self.demo_center label="center" on_toggle=self.link.callback(Msg::ToggleCenter)/></Column>
                    </Columns>
                    </Column>
                    <Column style="text-align:center">
                    <div class="box">
                       <Buttons addons=self.demo_addons center=self.demo_center right=self.demo_right>
                           <Button>{"Click Me!"}</Button>
                           <Button>{"Click Me"}</Button>
                      </Buttons>
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
                     <PropsTable<ButtonsProps> />
                 </DemoContainer>
               </Column>
              </Columns>
            </Container>
        }
    }
}
