use yew::prelude::*;
use yewlma::prelude::*;
use yew_route_breadcrumbs::Crumb;
use yewlma::components::BreadCrumbsProps;

use crate::components::{DemoContainer, PropertyInformation};

pub struct BreadCrumbsPage {
    link: ComponentLink<Self>,
    demo_size: Option<Size>,
}

pub enum Msg {
    SizeChanged(DropDownItem),
}

impl Component for BreadCrumbsPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        BreadCrumbsPage {
            link,
            demo_size: None,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::SizeChanged(size) => match size {
                DropDownItem::Item { text } => match text.as_ref() {
                    "none" => self.demo_size = None,
                    "small" => self.demo_size = Some(Size::Small),
                    "medium" => self.demo_size = Some(Size::Medium),
                    "large" => self.demo_size = Some(Size::Large),
                    _ => {}
                },
                _ => {}
            },
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
                          <h3>{"Size"}</h3>
                          <DropDownMenu items=sized_drop_down_items() placeholder="Size" onchange=self.link.callback(Msg::SizeChanged) />
                        </Column>
                    <Column style="text-align:center">
                    <div class="box">
                    <BreadCrumbs size=self.demo_size crumbs=breadcrumbs() />
                    </div>
                    </Column>
                    </Columns>
                    </DemoContainer>
                 </Column>
              </Columns>
              <PropertyInformation<BreadCrumbsProps> />
            </Container>
        }
    }
}

fn breadcrumbs() -> Vec<Crumb> {
    vec![
        Crumb {
            text: "One",
            route: None,
        },
        Crumb {
            text: "Two",
            route: None,
        },
        Crumb {
            text: "Three",
            route: None,
        },
    ]
}

fn sized_drop_down_items() -> Vec<DropDownItem> {
    vec![
        DropDownItem::Item {
            text: "none".into(),
        },
        DropDownItem::Item {
            text: "small".into(),
        },
        DropDownItem::Item {
            text: "medium".into(),
        },
        DropDownItem::Item {
            text: "large".into(),
        },
    ]
}
