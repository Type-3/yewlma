use yew::prelude::*;
use yewlma::prelude::*;
use yewtil::NeqAssign;

pub struct DemoContainer(Props);

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Props {
    pub children: Children,
}

impl Component for DemoContainer {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        DemoContainer(props)
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.0.neq_assign(props)
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="message is-primary">
              <div class="message-body">
                <Container>
                   { self.0.children.clone() }
                </Container>
              </div>
            </div>
        }
    }
}
