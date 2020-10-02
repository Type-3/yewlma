use yew::prelude::*;
use yewtil::NeqAssign;
use crate::prelude::*;
use yew_property_info::PropertyInfo;

pub struct Tabs {
    link: ComponentLink<Self>,
    props: TabsProps,
    index: usize
}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct TabsProps {
    pub titles: Vec<String>,
    #[prop_or(0)]
    pub current: usize,
    #[prop_or_default]
    pub center: bool,
    #[prop_or_default]
    pub right: bool,
    #[prop_or_default]
    pub boxed: bool,
    #[prop_or_default]
    pub toggle: bool,
    #[prop_or_default]
    pub toggle_round: bool,
    #[prop_or_default]
    pub fullwidth: bool,
    #[prop_or_default]
    pub children: Children
}

pub enum Msg {
    Selected(usize)
}

impl Component for Tabs {
    type Properties = TabsProps;
    type Message = Msg;

    fn create(props: TabsProps, link: ComponentLink<Self>) -> Self {
        Self { link, index: props.current, props }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.index = props.current;
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Selected(name) => self.index = name
        }
        true
    }

    fn view(&self) -> Html {
        let right = self.props.right.then_some("is-right");
        let center = self.props.center.then_some("is-center");
        let boxed = self.props.boxed.then_some("is-boxed");
        let exclusive = self.props.toggle.then_some("is-toggle");
        let fullwidth = self.props.fullwidth.then_some("is-fullwidth");
        let toggle_round = self.props.toggle_round.then_some("is-toggle-rounded");
        html! {
            <Container>
            <div class=("tabs", right, center, boxed, exclusive, fullwidth, toggle_round)>
                <ul>
                    {
                        for self.props.titles.iter().enumerate().map(|(dex, item)| {
                            let active = (dex == self.index).then_some("is-active");
                            yew::services::ConsoleService::error(&format!("dex: {}, index: {}, item: {:?}, active: {:?}", dex, self.index, item, active));
                            html! { <li class=active><a onclick=self.link.callback(move |_|Msg::Selected(dex))>{item}</a></li> }
                        })
                    }
                </ul>
            </div>
            <Container>
                {
                    for self.props.children.iter().skip(self.index).take(1).map(|child| {
                        child
                    })
                }
            </Container>
            </Container>
        }
    }
}
