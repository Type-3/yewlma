use yew::prelude::*;
use yewlma::prelude::*;
use yewtil::NeqAssign;

use crate::routes::*;
use crate::layouts::*;
use crate::components::DemoNavBar;

pub struct DocumentationLayout(Props, ComponentLink<Self>);

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Props {
    pub route: DocsRoutes
}

impl Component for DocumentationLayout {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        DocumentationLayout(props, link)
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.0.neq_assign(props)
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
                {
                    match self.0.route.clone() {
                            DocsRoutes::Components(route) => html! { <ComponentsLayout route=route /> },
                            DocsRoutes::Elements(route) => html ! { <ElementsLayout route=route /> },
                            DocsRoutes::Forms(route) => html! { <FormsLayout route=route  /> }
                    }
                }
            </Container>
            </div>
        }
    }
}
