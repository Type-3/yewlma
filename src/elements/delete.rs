use crate::prelude::*;
use yew::prelude::*;
use yew_property_info::PropertyInfo;
use yewtil::NeqAssign;

pub struct Delete(DeleteProps, ComponentLink<Self>);

pub enum Msg {
    Clicked,
}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct DeleteProps {
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub onsignal: Callback<()>,
}

impl Component for Delete {
    type Message = Msg;
    type Properties = DeleteProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Delete(props, link)
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.0.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => self.0.onsignal.emit(()),
        }
        false
    }

    fn view(&self) -> Html {
        let size = self.0.size.is();
        let color = self.0.color.has_bg();
        html! {
            <a onclick=self.1.callback(|_| Msg::Clicked)
               class=("delete", size, color, &self.0.class)></a>
        }
    }
}
