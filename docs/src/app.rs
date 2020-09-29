use crate::pages::*;
use crate::routes::*;
use crate::layouts::*;
use yew::prelude::*;
use yew_router::prelude::*;

pub struct App(ComponentLink<Self>);

impl Component for App {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, link: ComponentLink<Self>) -> Self {
        App(link)
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div>
            <Router<AppRoutes>
            render = Router::render(|switch| {
                match switch {
                    AppRoutes::Index => html! { <IndexPage />},
                    AppRoutes::Docs(route) => html! { <DocumentationLayout route=route /> }
                }
            })
            />
            </div>
        }
    }
}
