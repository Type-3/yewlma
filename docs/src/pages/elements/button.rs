use crate::components::{DemoContainer, PropsTable};
use crate::routes::AppRoutes;
use yew::prelude::*;
use yewlma::prelude::*;

pub struct ButtonPage {
    link: ComponentLink<Self>,
    demo_color: Option<Color>,
    demo_fg_color: Option<Color>,
    demo_size: Option<Size>,
    demo_outline: bool,
    demo_light: bool,
    demo_round: bool,
    demo_disabled: bool,
    demo_loading: bool,
    demo_inverted: bool,
}

pub enum Msg {
    ColorChanged(DropDownItem),
    FgColorChanged(DropDownItem),
    SizeChanged(DropDownItem),
    ToggleOutline(bool),
    ToggleLight(bool),
    ToggleRound(bool),
    ToggleDisabled(bool),
    ToggleInverted(bool),
    ToggleLoading(bool),
}

impl Component for ButtonPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        ButtonPage {
            link,
            demo_color: None,
            demo_fg_color: None,
            demo_outline: false,
            demo_light: false,
            demo_round: false,
            demo_disabled: false,
            demo_size: None,
            demo_loading: false,
            demo_inverted: false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleOutline(value) => self.demo_outline = value,
            Msg::ToggleLight(value) => self.demo_light = value,
            Msg::ToggleRound(value) => self.demo_round = value,
            Msg::ToggleDisabled(value) => self.demo_disabled = value,
            Msg::ToggleLoading(value) => self.demo_loading = value,
            Msg::ToggleInverted(value) => self.demo_inverted = value,
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
            Msg::FgColorChanged(item) => match item {
                DropDownItem::Item { text } => match text.as_ref() {
                    "none" => self.demo_fg_color = None,
                    "white" => self.demo_fg_color = Some(Color::White),
                    "black" => self.demo_fg_color = Some(Color::Black),
                    "danger" => self.demo_fg_color = Some(Color::Danger),
                    "warning" => self.demo_fg_color = Some(Color::Warning),
                    "info" => self.demo_fg_color = Some(Color::Info),
                    "success" => self.demo_fg_color = Some(Color::Success),
                    "primary" => self.demo_fg_color = Some(Color::Primary),
                    "dark" => self.demo_fg_color = Some(Color::Dark),
                    "light" => self.demo_fg_color = Some(Color::Light),
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
                          <h3>{"Text Color"}</h3>
                          <DropDownMenu items=color_dropdown_items() placeholder="Text" onchange=self.link.callback(Msg::FgColorChanged) />
                        </Column>
                        <Column>
                          <h3>{"Size"}</h3>
                          <DropDownMenu items=sized_drop_down_items() placeholder="Size" onchange=self.link.callback(Msg::SizeChanged) />
                        </Column>
                      </Columns>
                    <Columns class="py-3 px-2">
                        <Column><CheckBoxField value=self.demo_outline label="outlined" on_toggle=self.link.callback(Msg::ToggleOutline)/></Column>
                        <Column><CheckBoxField value=self.demo_light label="light" on_toggle=self.link.callback(Msg::ToggleLight)/></Column>
                        <Column><CheckBoxField value=self.demo_inverted label="inverted" on_toggle=self.link.callback(Msg::ToggleInverted)/></Column>
                    </Columns>
                    <Columns class="py-3 px-2">
                        <Column><CheckBoxField value=self.demo_round label="round" on_toggle=self.link.callback(Msg::ToggleRound)/></Column>
                        <Column><CheckBoxField value=self.demo_disabled label="disabled" on_toggle=self.link.callback(Msg::ToggleDisabled)/></Column>
                        <Column><CheckBoxField value=self.demo_loading label="loading" on_toggle=self.link.callback(Msg::ToggleLoading)/></Column>
                    </Columns>
                    </Column>
                    <Column style="text-align:center">
                    <div class="box">
                    <Button color=self.demo_color
                        light=self.demo_light
                        rounded=self.demo_round
                        fg=self.demo_fg_color
                        size=self.demo_size
                        inverted=self.demo_inverted
                        loading=self.demo_loading
                        disabled=self.demo_disabled
                        outlined=self.demo_outline>{"Click Me!"}
                    </Button>
                    </div>
                    </Column>
                    </Columns>
                    <Columns>
                      <Column class="box is-size-7">{self.render_example()}</Column>
                    </Columns>
                    </DemoContainer>
                 </Column>
              </Columns>
              <Columns>
                <span class="title py-5 px-5">{"Yew Router Support"}</span>
              </Columns>
              <Columns>
                <Column>
                  <DemoContainer>
                  <Columns class="is-vcentered">
                  <Column>
                    <p class="is-size-7">
                        <span class="tag is-danger m-3">{"Warning"}</span>
                        {"In order to pass the route property you must specify the Switch type."}
                        <div class="box is-size-7">{"<Button<Routes> route=Routes::SomeRoute>{\"Click me!\"}</Button<Routes>>"}</div>
                    </p>
                    </Column>
                    <Column style="text-align:center">
                      <div class="box"><Button<AppRoutes> color=Color::Primary route=AppRoutes::Index>{"Go To Index"}</Button<AppRoutes>></div>
                    </Column>
                    </Columns>
                  </DemoContainer>
                </Column>
              </Columns>
              <Columns>
              <Column>
                 <h1 class="title">{"Properties"}</h1>
                 <DemoContainer>
                     <PropsTable<ButtonProps> />
                 </DemoContainer>
               </Column>
              </Columns>
            </Container>
        }
    }
}

impl ButtonPage {
    fn render_example(&self) -> String {
        let mut attrs = vec![];
        if self.demo_color.is_some() {
            attrs.push(("color", format!("Color::{}", self.demo_color.unwrap())));
        }
        if self.demo_fg_color.is_some() {
            attrs.push(("color", format!("Color::{}", self.demo_fg_color.unwrap())));
        }
        if self.demo_size.is_some() {
            attrs.push(("size", format!("Size::{}", self.demo_size.unwrap())));
        }
        if self.demo_light {
            attrs.push(("light", "true".into()));
        }
        if self.demo_outline {
            attrs.push(("outlined", "true".into()));
        }
        if self.demo_loading {
            attrs.push(("loading", "true".into()));
        }
        if self.demo_disabled {
            attrs.push(("disabled", "true".into()));
        }
        if self.demo_inverted {
            attrs.push(("inverted", "true".into()));
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
