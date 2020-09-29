use yew::prelude::*;
use yewlma::prelude::*;
use yewlma::forms::InputFieldProps;
use crate::components::{DemoContainer, PropsTable};

pub struct InputPage {
    link: ComponentLink<Self>,
    demo_color: Option<Color>,
    demo_size: Option<Size>,
    demo_round: bool,
    demo_disabled: bool,
    demo_loading: bool,
}

pub enum Msg {
    ColorChanged(DropDownItem),
    SizeChanged(DropDownItem),
    ToggleRound(bool),
    ToggleDisabled(bool),
    ToggleLoading(bool),
}

impl Component for InputPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        InputPage {
            link,
            demo_color: None,
            demo_round: false,
            demo_disabled: false,
            demo_size: None,
            demo_loading: false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleRound(value) => self.demo_round = value,
            Msg::ToggleDisabled(value) => self.demo_disabled = value,
            Msg::ToggleLoading(value) => self.demo_loading = value,
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
                        <Column><CheckBoxField value=self.demo_round label="round" on_toggle=self.link.callback(Msg::ToggleRound)/></Column>
                        <Column><CheckBoxField value=self.demo_disabled label="disabled" on_toggle=self.link.callback(Msg::ToggleDisabled)/></Column>
                        <Column><CheckBoxField value=self.demo_loading label="loading" on_toggle=self.link.callback(Msg::ToggleLoading)/></Column>
                    </Columns>
                    </Column>
                    <Column style="text-align:center">
                    <div class="box">
                    <InputField color=self.demo_color
                        rounded=self.demo_round
                        size=self.demo_size
                        loading=self.demo_loading
                        disabled=self.demo_disabled/>
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
                     <PropsTable<InputFieldProps> />
                 </DemoContainer>
               </Column>
              </Columns>
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
