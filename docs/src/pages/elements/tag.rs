use crate::components::{DemoContainer, PropertyInformation};
use yew::prelude::*;
use yewlma::elements::TagProps;
use yewlma::prelude::*;

pub struct TagPage {
    link: ComponentLink<Self>,
    demo_color: Option<Color>,
    demo_size: Option<Size>,
    demo_light: bool,
    demo_round: bool,
}

pub enum Msg {
    ColorChanged(DropDownItem),
    SizeChanged(DropDownItem),
    ToggleLight(bool),
    ToggleRound(bool),
}

impl Component for TagPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        TagPage {
            link,
            demo_color: None,
            demo_light: false,
            demo_round: false,
            demo_size: None,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleLight(value) => self.demo_light = value,
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
                      <Columns>
                        <Column>
                          <h3>{"Color"}</h3>
                          <DropDownMenu items=color_dropdown_items() placeholder="Color" onchange=self.link.callback(Msg::ColorChanged) />
                        </Column>
                        <Column>
                          <h3>{"Size"}</h3>
                          <DropDownMenu items=sized_drop_down_items() placeholder="Size" onchange=self.link.callback(Msg::SizeChanged) />
                        </Column>
                      </Columns>
                    <Columns class="py-3 px-2">
                        <Column><CheckBoxField value=self.demo_light label="light" on_toggle=self.link.callback(Msg::ToggleLight)/></Column>
                        <Column><CheckBoxField value=self.demo_round label="round" on_toggle=self.link.callback(Msg::ToggleRound)/></Column>
                    </Columns>
                    </Column>
                    <Column style="text-align:center">
                    <div class="box">
                    <Tag color=self.demo_color
                        light=self.demo_light
                        rounded=self.demo_round
                        size=self.demo_size>{"Tag Text"}
                    </Tag>
                    </div>
                    </Column>
                    </Columns>
                    <Columns>
                      <Column class="box is-size-7">{self.render_example()}</Column>
                    </Columns>
                    </DemoContainer>
                 </Column>
              </Columns>
              <PropertyInformation<TagProps> />
            </Container>
        }
    }
}

impl TagPage {
    fn render_example(&self) -> String {
        let mut attrs = vec![];
        if self.demo_color.is_some() {
            attrs.push(("color", format!("Color::{}", self.demo_color.unwrap())));
        }
        if self.demo_size.is_some() {
            attrs.push(("size", format!("Size::{}", self.demo_size.unwrap())));
        }
        if self.demo_light {
            attrs.push(("light", "true".into()));
        }
        if self.demo_round {
            attrs.push(("rounded", "true".into()));
        }
        let mut output = format!("<Button");
        for attr in attrs {
            output += &format!(" {}={}", attr.0, attr.1);
        }
        output += ">{\"Click me!\"}</Button>";
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
