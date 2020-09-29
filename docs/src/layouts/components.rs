use yew::prelude::*;
use yewtil::NeqAssign;
use yewlma::prelude::*;
use yew_router::components::RouterAnchor;
use crate::pages::{DropDownPage, ComponentsIndex, BreadCrumbsPage, PaginationPage};
use crate::routes::*;

pub struct ComponentsLayout {
    props: Props,
    _link: ComponentLink<Self>
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Props {
    pub route: ComponentsRoutes
}

impl Component for ComponentsLayout {
    type Message = ();
    type Properties = Props;

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
        html! {
            <Container>
            <Columns>
            <Column narrow=Narrow(None)>
            <aside class="menu mt-3">
              <p class="menu-label">{"Components"}</p>
              <ul class="menu-list">
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Components(ComponentsRoutes::DropDown))>{"DropDowns"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Components(ComponentsRoutes::BreadCrumbs))>{"BreadCrumbs"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Components(ComponentsRoutes::Pagination))>{"Pagination"}</RouterAnchor<AppRoutes>></li>
              </ul>
            </aside>
            </Column>
            <Column>
            <Container class="mt-2">
            <Columns>
            <Column><BreadCrumbs crumbs=self.props.route.breadcrumbs() size=Size::Medium /></Column>
            </Columns>
            <Columns>
            <Column>
            {
                match self.props.route {
                    ComponentsRoutes::Index => html! {<ComponentsIndex />},
                    ComponentsRoutes::DropDown => html! { <DropDownPage /> },
                    ComponentsRoutes::BreadCrumbs => html! { <BreadCrumbsPage /> },
                    ComponentsRoutes::Pagination => html! { <PaginationPage /> }
                }
            }
            </Column>
            </Columns>
            </Container>
            </Column>
            </Columns>
            </Container>
        }
    }
}
