use yew::prelude::*;
use yew_router::agent::RouteRequest;
use yew_router::prelude::*;
use yew_router::Switch;
use yew_property_info::PropertyInfo;

use crate::classes::{Color, CssRepr, Size};
use crate::utils::{NullSwitchImplementor, SwitchImplementor};

pub enum Msg {
    Clicked,
}

pub struct Button<SW: SwitchImplementor = NullSwitchImplementor, STATE: RouterState = ()> {
    props: ButtonProps<SW>,
    router: RouteAgentDispatcher<STATE>,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, PartialEq, Properties, PropertyInfo)]
pub struct ButtonProps<SW: Switch + Clone + 'static = NullSwitchImplementor> {
    #[prop_or_default]
    #[prop_description("Value of the HTML id field")]
    pub id: Option<String>,
    #[prop_or("button")]
    #[prop_description("HTML tag to use as the root element")]
    pub tag: &'static str,
    #[prop_or_default]
    #[prop_description("The color of the button")]
    pub color: Option<Color>,
    #[prop_or_default]
    #[prop_description("The foreground color of the button")]
    pub fg: Option<Color>,
    #[prop_or_default]
    #[prop_description("Sets the is-light modifier")]
    pub light: bool,
    #[prop_or_default]
    #[prop_description("Sets the is-outlined modifier")]
    pub outlined: bool,
    #[prop_or_default]
    #[prop_description("Sets the is-inverted modifier")]
    pub inverted: bool,
    #[prop_or_default]
    #[prop_description("Sets the is-rounded modifier")]
    pub rounded: bool,
    #[prop_or_default]
    #[prop_description("Sets the is-loading modifier")]
    pub loading: bool,
    #[prop_or_default]
    #[prop_description("Sets the  HTML disabled field")]
    pub disabled: bool,
    #[prop_or_default]
    #[prop_description("Size of the button")]
    pub size: Option<Size>,
    #[prop_or_default]
    #[prop_description("Sets the is-fullwidth modifier")]
    pub fullwidth: bool,
    #[prop_or_default]
    #[prop_description("Classes to apply to the root element")]
    pub class: Option<String>,
    #[prop_or_default]
    #[prop_description("Callback called when the button is clicked")]
    pub onsignal: Callback<()>,
    #[prop_or_default]
    #[prop_description("The inner HTML elements")]
    pub children: Children,
    #[prop_or_default]
    #[prop_description("Route to change to when the button is clicked")]
    pub route: Option<SW>,
}

impl<SW: Switch + Clone + 'static, STATE: RouterState> Component for Button<SW, STATE> {
    type Message = Msg;
    type Properties = ButtonProps<SW>;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let router = RouteAgentDispatcher::new();
        Button {
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
        let fg = self.props.fg.has_text();
        let size = self.props.size.is();
        let color = self.props.color.is();
        let light = self.props.light.then_some("is-light");
        let outlined = self.props.outlined.then_some("is-outlined");
        let inverted = self.props.inverted.then_some("is-inverted");
        let rounded = self.props.rounded.then_some("is-rounded");
        let loading = self.props.loading.then_some("is-loading");
        let fullwidth = self.props.fullwidth.then_some("is-fullwidth");
        html! {
            <@{self.props.tag}
                 id?=self.props.id.as_ref()
                 disabled=self.props.disabled
                 onclick=self.link.callback(|_| Msg::Clicked)
                 class=(
                     "button", color, size, fg,
                     light, outlined, inverted, fullwidth,
                     rounded, loading, &self.props.class
                 )>
                 { self.props.children.clone() }
            </@>
        }
    }
}
