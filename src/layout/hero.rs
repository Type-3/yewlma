use yew::prelude::*;
use crate::prelude::*;
use yewtil::NeqAssign;
use yew_property_info::PropertyInfo;

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
        html! {
            <@{self.props.tag} class=("hero", color, bold, &self.props.class)>
                <div class="hero-body">
                  <Container>
                    {self.props.children.clone()}
                  </Container>
                </div>
            </@>
        }
    }
}
