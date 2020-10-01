use crate::pages::{ColumnPage, ColumnsPage, ContainerPage, LayoutsIndex};
use crate::routes::*;
use yew::prelude::*;
use yew_router::components::RouterAnchor;
use yewlma::prelude::*;
use yewtil::NeqAssign;

pub struct LayoutsLayout {
    props: Props,
    _link: ComponentLink<Self>,
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Props {
    pub route: LayoutRoutes,
}

impl Component for LayoutsLayout {
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
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Layout(LayoutRoutes::Container))>{"Container"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Layout(LayoutRoutes::Columns))>{"Columns"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Layout(LayoutRoutes::Column))>{"Column"}</RouterAnchor<AppRoutes>></li>
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
                    LayoutRoutes::Index => html! {<LayoutsIndex />},
                    LayoutRoutes::Container => html! { <ContainerPage /> },
                    LayoutRoutes::Columns => html! { <ColumnsPage /> },
                    LayoutRoutes::Column => html! { <ColumnPage /> }
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
