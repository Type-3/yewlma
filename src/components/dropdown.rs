use yew::prelude::*;
use yewtil::NeqAssign;
use yew_property_info::PropertyInfo;

use crate::prelude::*;

#[derive(Debug, PartialEq, Clone)]
pub enum DropDownItem {
    Divider,
    Item { text: String },
}

#[derive(Debug, PartialEq)]
pub enum Msg {
    Toggle,
    Selected(usize),
}

pub struct DropDownMenu {
    pub active: bool,
    pub selected: Option<usize>,
    pub props: DropDownMenuProps,
    pub link: ComponentLink<Self>,
}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct DropDownMenuProps {
    pub items: Vec<DropDownItem>,
    #[prop_or_default]
    pub trigger_color: Option<Color>,
    #[prop_or_default]
    pub trigger_light: bool,
    #[prop_or_default]
    pub trigger_rounded: bool,
    #[prop_or_default]
    pub trigger_outlined: bool,
    #[prop_or_default]
    pub trigger_size: Option<Size>,
    #[prop_or("Please Select".to_string())]
    pub placeholder: String,
    #[prop_or("fa fa-angle-left".to_string())]
    pub icon: String,
    #[prop_or("fa fa-angle-down".to_string())]
    pub active_icon: String,
    #[prop_or_default]
    pub icon_color: Option<Color>,
    #[prop_or(true)]
    pub close_on_change: bool,
    #[prop_or_default]
    pub onchange: Callback<DropDownItem>,
}

impl Component for DropDownMenu {
    type Message = Msg;
    type Properties = DropDownMenuProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        DropDownMenu {
            active: false,
            selected: None,
            props,
            link,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Toggle => self.active = !self.active,
            Msg::Selected(opt) => {
                if self.props.close_on_change {
                    self.active = false;
                }
                self.selected = Some(opt);
                self.props.onchange.emit(self.props.items[opt].clone())
            }
        }
        true
    }

    fn view(&self) -> Html {
        let active = self.active.then_some("is-active");
        let icon = if self.active {
            &self.props.active_icon
        } else {
            &self.props.icon
        };
        html! {
            <div class=("dropdown", active)>
              <div class="dropdown-trigger">
                <Button onsignal=self.link.callback(|_| Msg::Toggle)
                        rounded=self.props.trigger_rounded
                        outlined=self.props.trigger_outlined
                        light=self.props.trigger_light
                        size=self.props.trigger_size
                        color=self.props.trigger_color><span>{
                            if let Some(selected) = self.selected {
                                html! {
                                    {
                                        if let DropDownItem::Item { text } = &self.props.items[selected] {
                                            &text
                                        } else {
                                            &self.props.placeholder
                                        }
                                    }
                                }
                            } else {
                                html! { &self.props.placeholder }
                            }
                        }</span>
                        <Icon size=Size::Small
                              color=self.props.icon_color
                              name={icon} />
                        </Button>
              </div>
              <div class="dropdown-menu">
                <div class="dropdown-content">
                  {
                      for self.props.items
                        .iter()
                        .enumerate()
                        .map(|(dex, item)| {
                           match item {
                             DropDownItem::Divider => html_nested!{
                                 <div class="dropdown-divider"></div>
                             },
                             DropDownItem::Item { text } => {
                                 let class = if self.selected == Some(dex) {
                                     "dropdown-item is-active"
                                 } else if self.selected == None && text == "10" {
                                     "dropdown-item is-active"
                                 } else {
                                     "dropdown-item".into()
                                 };
                                   html_nested! {
                                      <a class=class
                                         onclick=&self.link.callback(move |_| Msg::Selected(dex))>
                                         {text}
                                      </a>
                                      }
                                    }
                                  }
                            })

                  }
                </div>
              </div>
            </div>
        }
    }
}
