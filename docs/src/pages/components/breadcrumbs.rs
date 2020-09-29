use yewlma::prelude::*;
use yew::prelude::*;
use yew_route_breadcrumbs::Crumb;
use yewlma::components::BreadCrumbsProps;
use crate::components::{DemoContainer, PropsTable};

pub struct BreadCrumbsPage;

pub enum Msg {}

impl Component for BreadCrumbsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        BreadCrumbsPage
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
                    <BreadCrumbs crumbs=breadcrumbs() />
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
                     <PropsTable<BreadCrumbsProps> />
                 </DemoContainer>
               </Column>
              </Columns>
            </Container>
        }
    }
}

fn breadcrumbs() -> Vec<Crumb> {
    vec![
        Crumb {
            text: "One",
            route: None
        },
        Crumb {
            text: "Two",
            route: None
        },
        Crumb {
            text: "Three",
            route: None
        }
    ]
}
