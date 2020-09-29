use crate::components::DemoNavBar;
use crate::pages::*;
use crate::routes::*;
use crate::layouts::*;
use yewlma::prelude::*;
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
            <DemoNavBar />
            <Toaster />
            <Container>
                <Router<AppRoutes>
                    render = Router::render(|switch| {
                        match switch {
                            AppRoutes::Index => html! { <IndexPage />},
                            AppRoutes::Components(route) => html! { <ComponentsLayout route=route /> },
                            AppRoutes::Elements(route) => html ! { <ElementsLayout route=route /> },
                            AppRoutes::Forms(route) => html! { <FormsLayout route=route  /> }
                        }
                    })
                />
            </Container>
            </div>
        }
    }
}
