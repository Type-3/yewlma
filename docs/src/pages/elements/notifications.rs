use crate::components::{DemoContainer, PropsTable};
use yew::prelude::*;
use yewlma::elements::NotificationProps;
use yewlma::prelude::*;

pub struct NotificationPage {
    link: ComponentLink<Self>,
    demo_color: Option<Color>,
    demo_light: bool,
    demo_delete: bool,
}

pub enum Msg {
    ColorChanged(DropDownItem),
    ToggleLight(bool),
    ToggleDelete(bool),
}

impl Component for NotificationPage {
    type Message = Msg;
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        NotificationPage {
            link,
            demo_color: None,
            demo_light: false,
            demo_delete: false,
        }
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ToggleLight(value) => self.demo_light = value,
            Msg::ToggleDelete(value) => self.demo_delete = value,
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
                      <Columns class="py-3 px-2 is-vcentered">
                        <Column>
                          <DropDownMenu items=color_dropdown_items() placeholder="Color" onchange=self.link.callback(Msg::ColorChanged) />
                        </Column>
                        <Column><CheckBoxField value=self.demo_light label="light" on_toggle=self.link.callback(Msg::ToggleLight)/></Column>
                        <Column><CheckBoxField value=self.demo_delete label="delete" on_toggle=self.link.callback(Msg::ToggleDelete)/></Column>
                    </Columns>
                    </Column>
                    <Column style="text-align:center">
                    <div class="box m-1">
                    <Notification color=self.demo_color
                        delete=self.demo_delete
                        light=self.demo_light>{"Here is something you need to know!"}
                    </Notification>
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
              <Column>
                 <h1 class="title">{"Properties"}</h1>
                 <DemoContainer>
                     <PropsTable<NotificationProps>  />
                 </DemoContainer>
               </Column>
              </Columns>
            </Container>
        }
    }
}

impl NotificationPage {
    fn render_example(&self) -> String {
        let mut attrs = vec![];
        if self.demo_color.is_some() {
            attrs.push(("color", format!("Color::{}", self.demo_color.unwrap())));
        }
        if self.demo_light {
            attrs.push(("light", "true".into()));
        }
        if self.demo_delete {
            attrs.push(("delete", "true".into()));
        }
        let mut output = format!("<Notification");
        for attr in attrs {
            output += &format!(" {}={}", attr.0, attr.1);
        }
        output += ">{\"Message Text\"}</>";
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
