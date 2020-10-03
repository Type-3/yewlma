use yew::prelude::*;
use crate::prelude::*;
use yewtil::NeqAssign;
use yew_property_info::PropertyInfo;

mod head;
pub use self::head::{HeroHead, HeroHeadProps};
mod foot;
pub use self::foot::{HeroFoot, HeroFootProps};

pub struct Hero {
    _link: ComponentLink<Self>,
    props: HeroProps
}

use yew::html::ChildrenRenderer;
use yew::virtual_dom::VChild;

#[derive(Debug, PartialEq, Clone)]
pub enum HeroChild {
    Head(VChild<HeroHead>),
    Foot(VChild<HeroFoot>),
    Other(Html)
}

impl Into<Html> for HeroChild {
    fn into(self) -> Html {
        match self {
            HeroChild::Head(head) => head.into(),
            HeroChild::Foot(foot) => foot.into(),
            HeroChild::Other(other) => other
//            Variants::Header(props) => {
//                VComp::new::<ListHeader>(props, NodeRef::default(), None).into()
//            }
//            Variants::Item(props) => VComp::new::<ListItem>(props, NodeRef::default(), None).into(),
        }
    }
}

impl From<VChild<HeroHead>> for HeroChild {
  fn from(value: VChild<HeroHead>) -> Self {
    Self::Head(value)
  }
}

impl From<VChild<HeroFoot>> for HeroChild {
  fn from(value: VChild<HeroFoot>) -> Self {
    Self::Foot(value)
  }
}

impl From<Html> for HeroChild {
  fn from(value: Html) -> Self {
    Self::Other(value)
  }
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
    pub children: ChildrenRenderer<HeroChild>
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
        html! {
            <@{self.props.tag} class=("hero", color, bold, fullheight, &self.props.class)>
               {self.render_head_child()}
                <div class="hero-body">
                  <Container>
                      {self.render_other_children()}
                  </Container>
                </div>
                {self.render_foot_child()}
            </@>
        }
    }
}

impl Hero {

    pub fn render_head_child(&self) -> Html {
        html! { for self.props.children.iter().filter(|item| matches!(item, HeroChild::Head(_))) }
    }

    pub fn render_foot_child(&self) -> Html {
        html! { for self.props.children.iter().filter(|item| matches!(item, HeroChild::Foot(_))) }
    }

    pub fn render_other_children(&self) -> Html {
        html! { for self.props.children.iter().filter(|item|matches!(item, HeroChild::Other(_))) }
    }
}
