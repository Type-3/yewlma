use crate::routes::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod dropdown;
pub use self::dropdown::DropDownPage;
mod breadcrumbs;
pub use self::breadcrumbs::BreadCrumbsPage;
mod pagination;
pub use self::pagination::PaginationPage;

pub struct ComponentsIndex;

impl Component for ComponentsIndex {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        Self
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Container>
            <span class="title">{"TODO: Make index page"}</span>
            <ul>
                 <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Components(ComponentsRoutes::BreadCrumbs))>{"BreadCrumbs"}</RouterAnchor<AppRoutes>></li>
                 <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Components(ComponentsRoutes::DropDown))>{"DropDown"}</RouterAnchor<AppRoutes>></li>
                 <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Components(ComponentsRoutes::Pagination))>{"Pagination"}</RouterAnchor<AppRoutes>></li>
            </ul>
            </Container>
        }
    }
}
