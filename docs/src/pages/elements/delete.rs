use yew::prelude::*;
use yewlma::elements::DeleteProps;
use yewlma::prelude::*;

use crate::components::{DemoContainer, PropertyInformation};

pub struct DeletePage {
    link: ComponentLink<Self>,
    demo_color: Option<Color>,
    demo_size: Option<Size>,
}

pub enum Msg {
    ColorChanged(DropDownItem),
    SizeChanged(DropDownItem),
}

impl Component for DeletePage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        DeletePage {
            link,
            demo_color: None,
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
                      <Columns>
                        <Column>
                          <DropDownMenu items=color_dropdown_items() placeholder="Color" onchange=self.link.callback(Msg::ColorChanged) />
                        </Column>
                        <Column>
                          <DropDownMenu items=sized_drop_down_items() placeholder="Size" onchange=self.link.callback(Msg::SizeChanged) />
                        </Column>
                      </Columns>
                    </Column>
                    <Column style="text-align:center">
                    <div class="box">
                      <Delete color=self.demo_color size=self.demo_size />
                    </div>
                    </Column>
                    </Columns>
                    <Columns class="mt-1">
                      <Column class="box is-size-7">{self.render_example()}</Column>
                    </Columns>
                    </DemoContainer>
                 </Column>
              </Columns>
              <PropertyInformation<DeleteProps> />
            </Container>
        }
    }
}

impl DeletePage {
    fn render_example(&self) -> String {
        let mut attrs = vec![];
        if self.demo_color.is_some() {
            attrs.push(("color", format!("Color::{}", self.demo_color.unwrap())));
        }
        if self.demo_size.is_some() {
            attrs.push(("size", format!("Size::{}", self.demo_size.unwrap())));
        }
        let mut output = format!("<Delete");
        for attr in attrs {
            output += &format!(" {}={}", attr.0, attr.1);
        }
        output += " />";
        output
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
