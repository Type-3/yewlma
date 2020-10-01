use crate::components::{DemoContainer, PropsTable};
use yew::prelude::*;
use yewlma::elements::TagsProps;
use yewlma::prelude::*;

pub struct TagsPage {
    link: ComponentLink<Self>,
    demo_addons: bool,
}

pub enum Msg {
    ToggleAddons(bool),
}

impl Component for TagsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TagsPage {
            link,
            demo_addons: false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleAddons(value) => self.demo_addons = value,
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
                    <Column><CheckBoxField value=self.demo_addons label="addons" on_toggle=self.link.callback(Msg::ToggleAddons)/></Column>
                    <Column style="text-align:center">
                    <div class="box">
                       <Tags addons=self.demo_addons>
                           <Tag color=Color::Primary>{"Click Me!"}</Tag>
                           <Tag color=Color::Success>{"Click Me"}</Tag>
                      </Tags>
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
                     <PropsTable<TagsProps> />
                 </DemoContainer>
               </Column>
              </Columns>
            </Container>
        }
    }
}
