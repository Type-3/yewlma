use yew::prelude::*;
use yewtil::NeqAssign;
use yew_property_info::PropertyInfo;

use crate::prelude::*;

pub struct Notification {
    props: NotificationProps,
    link: ComponentLink<Self>,
}

pub enum Msg {
    DeleteClicked,
}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct NotificationProps {
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub light: bool,
    #[prop_or(true)]
    pub delete: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub ondelete: Callback<()>,
    #[prop_or_default]
    pub children: Children,
}

impl Component for Notification {
    type Message = Msg;
    type Properties = NotificationProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Notification { props, link }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::DeleteClicked => self.props.ondelete.emit(()),
        }
        false
    }

    fn view(&self) -> Html {
        let color = self.props.color.is();
        let light = self.props.light.then_some("is-light");
        html! {
            <div class=("notification", color, light, &self.props.class)>
            {
                if self.props.delete {
                    html_nested! { <button class="delete" onclick=self.link.callback(|_| Msg::DeleteClicked)></button> }
                } else {
                    html_nested! {}
                }
            }
            { self.props.children.clone() }
            </div>
        }
    }
}
