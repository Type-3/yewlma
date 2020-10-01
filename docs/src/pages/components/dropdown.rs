use yew::prelude::*;
use yewlma::prelude::*;
use yewlma::components::DropDownMenuProps;

use crate::components::{PropertyInformation, DemoContainer};

pub struct DropDownPage {
    link: ComponentLink<Self>,
    demo_color: Option<Color>,
    demo_size: Option<Size>,
    demo_round: bool,
}

pub enum Msg {
    ColorChanged(DropDownItem),
    SizeChanged(DropDownItem),
    ToggleRound(bool),
}

impl Component for DropDownPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        DropDownPage {
            link,
            demo_color: None,
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
            Msg::ColorChanged(item) => match item {
                DropDownItem::Item { text } => match text.as_ref() {
                    "none" => self.demo_color = None,
                    "white" => self.demo_color = Some(Color::White),
                    "black" => self.demo_color = Some(Color::Black),
                    "danger" => self.demo_color = Some(Color::Danger),
                    "warning" => self.demo_color = Some(Color::Warning),
                    "info" => self.demo_color = Some(Color::Info),
                    "success" => self.demo_color = Some(Color::Success),
                    "primary" => self.demo_color = Some(Color::Primary),
                    "dark" => self.demo_color = Some(Color::Dark),
                    "light" => self.demo_color = Some(Color::Light),
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
                      <Columns class="is-vcentered">
                        <Column>
                          <h3>{"Color"}</h3>
                          <DropDownMenu items=color_dropdown_items() placeholder="Color" onchange=self.link.callback(Msg::ColorChanged) />
                        </Column>
                        <Column>
                          <h3>{"Size"}</h3>
                          <DropDownMenu items=sized_drop_down_items() placeholder="Size" onchange=self.link.callback(Msg::SizeChanged) />
                        </Column>
                        <Column><CheckBoxField value=self.demo_round label="round" on_toggle=self.link.callback(Msg::ToggleRound) /></Column>
                      </Columns>
                    </Column>
                    <Column style="text-align:center">
                    <div class="box">
                    <DropDownMenu
                        items=dropdown_items()
                        trigger_color=self.demo_color
                        trigger_rounded=self.demo_round
                        trigger_size=self.demo_size />
                    </div>
                    </Column>
                    </Columns>
                    </DemoContainer>
                 </Column>
              </Columns>
              <PropertyInformation<DropDownMenuProps> />
            </Container>
        }
    }
}

fn color_dropdown_items() -> Vec<DropDownItem> {
    let mut items: Vec<DropDownItem> = Color::all()
        .iter()
        .map(|item| DropDownItem::Item {
            text: format!("{}", item),
        })
        .collect();
    items.insert(
        0,
        DropDownItem::Item {
            text: "none".into(),
        },
    );
    items
}

fn dropdown_items() -> Vec<DropDownItem> {
    vec![
        DropDownItem::Item {
            text: "Item 1".into(),
        },
        DropDownItem::Item {
            text: "Item 2".into(),
        },
        DropDownItem::Divider,
        DropDownItem::Item {
            text: "Item 3".into(),
        },
        DropDownItem::Item {
            text: "Item 4".into(),
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
