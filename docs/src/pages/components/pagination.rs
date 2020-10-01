use yew::prelude::*;
use yewlma::prelude::*;
use yewlma::components::PaginationProps;

use crate::components::{DemoContainer, PropertyInformation};

pub struct PaginationPage {
    link: ComponentLink<Self>,
    demo_round: bool,
    demo_size: Option<Size>,
}

pub enum Msg {
    ToggleRound(bool),
    SizeChanged(DropDownItem),
}

impl Component for PaginationPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        PaginationPage {
            link,
            demo_round: false,
            demo_size: None,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleRound(value) => self.demo_round = value,
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
                        <Column narrow=Narrow(None)>
                          <h3>{"Size"}</h3>
                          <DropDownMenu items=sized_drop_down_items() placeholder="Size" onchange=self.link.callback(Msg::SizeChanged) />
                        </Column>
                        <Column narrow=Narrow(None)><CheckBoxField value=self.demo_round label="round" on_toggle=self.link.callback(Msg::ToggleRound) /></Column>
                        <Column style="text-align:center">
                            <div class="box">
                                 <Pagination total=200 per_page=10 page=5 round=self.demo_round size=self.demo_size />
                            </div>
                        </Column>
                    </Columns>
                    </DemoContainer>
                 </Column>
              </Columns>
              <PropertyInformation<PaginationProps> />
            </Container>
        }
    }
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
