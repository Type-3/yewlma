use yew::prelude::*;
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;
use yew_router::Switch;

use crate::utils::{NullSwitchImplementor, SwitchImplementor};

pub enum Msg {
    Clicked,
}

pub struct Brand<SW: SwitchImplementor = NullSwitchImplementor, STATE: RouterState = ()> {
    props: Props<SW>,
    router: RouteAgentDispatcher<STATE>,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, Properties)]
pub struct Props<SW: Switch + Clone + 'static> {
    #[prop_or_default]
    pub id: Option<String>,
    #[prop_or("div")]
    pub tag: &'static str,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub onsignal: Callback<()>,
    #[prop_or_default]
    pub children: Children,
    #[prop_or_default]
    pub route: Option<SW>,
}

impl<SW: Switch + Clone + 'static, STATE: RouterState> Component for Brand<SW, STATE> {
    type Message = Msg;
    type Properties = Props<SW>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();
        Brand {
            props,
            link,
            router,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props = props;
        true
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Clicked => {
                self.props.onsignal.emit(());
                if let Some(route) = &self.props.route {
                    let route = Route::from(route.clone());
                    self.router.send(RouteRequest::ChangeRoute(route));
                }
            }
        }
        false
    }

    fn view(&self) -> Html {
        html! {
            <@{self.props.tag}
                 id?=self.props.id.as_ref()
                 onclick=self.link.callback(|_| Msg::Clicked)
                 class=(
                     "navbar-brand", &self.props.class
                 )>
                 <a class="navbar-item" onclick=self.link.callback(|_| Msg::Clicked)>
                   { self.props.children.clone() }
                 </a>
            </@>
        }
    }
}
