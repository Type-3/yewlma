use yew::prelude::*;
use crate::prelude::*;
use yewtil::NeqAssign;
use yew_property_info::PropertyInfo;
use yew::virtual_dom::VNode;
use std::any::TypeId;

mod head;
pub use self::head::{HeroHead, HeroHeadProps};
mod foot;
pub use self::foot::{HeroFoot, HeroFootProps};

pub struct Hero {
    _link: ComponentLink<Self>,
    props: HeroProps
}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct HeroProps {
    #[prop_or("div")]
    pub tag: &'static str,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub bold: bool,
    #[prop_or_default]
    pub fullheight: bool,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub children: Children
}

impl Component for Hero {
    type Properties = HeroProps;
    type Message = ();

    fn create(props: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self { props, _link }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let color = self.props.color.is();
        let bold = self.props.bold.then_some("is-bold");
        let fullheight = self.props.fullheight.then_some("is-fullheight");
        let mut childs = self.props.children.clone().iter().collect::<Vec<VNode>>();
        let mut head_child = None;
        let mut foot_child = None;
        let mut head_index = None;
        let mut foot_index = None;
        for (dex, child) in childs.iter().enumerate() {
            yew::services::ConsoleService::log(&format!("{:?}", child));
            if let VNode::VComp(comp) = child {
                if comp.type_id == TypeId::of::<HeroHead>() {
                     head_index = Some(dex);
                } else if comp.type_id == TypeId::of::<HeroFoot>() {
                    foot_index = Some(dex);
                }
            }
        }
        if let Some(index) = head_index {
            head_child = Some(childs.remove(index));
        }
        if let Some(index) = foot_index {
            foot_child = Some(childs.remove(index));
        }
        html! {
            <@{self.props.tag} class=("hero", color, bold, fullheight, &self.props.class)>
                {
                    if let Some(comp) = head_child {
                        comp
                    } else {
                        html!{}
                    }
                }
                <div class="hero-body">
                  <Container>
                    {childs}
                  </Container>
                </div>
                {
                    if let Some(comp) = foot_child {
                        comp
                    } else {
                        html!{}
                    }
                }
            </@>
        }
    }
}
