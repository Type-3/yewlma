use yew::prelude::*;
use yew_router::prelude::*;
use yewlma::prelude::*;

use crate::routes::*;

mod container;
pub use self::container::ContainerPage;
mod columns;
pub use self::columns::ColumnsPage;
mod column;
pub use self::column::ColumnPage;
mod hero;
pub use self::hero::HeroPage;
mod footer;
pub use self::footer::FooterPage;

pub struct LayoutsIndex;

impl Component for LayoutsIndex {
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
            <Column>
            <span class="title">{"TODO: Make index page"}</span>
            <ul>
                 <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Layout(LayoutRoutes::Container))>{"Container"}</RouterAnchor<AppRoutes>></li>
                 <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Layout(LayoutRoutes::Column))>{"Column"}</RouterAnchor<AppRoutes>></li>
                 <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Layout(LayoutRoutes::Columns))>{"Columns"}</RouterAnchor<AppRoutes>></li>
                 <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Layout(LayoutRoutes::Hero))>{"Hero"}</RouterAnchor<AppRoutes>></li>
                 <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Layout(LayoutRoutes::Footer))>{"Footer"}</RouterAnchor<AppRoutes>></li>
            </ul>
            </Column>
            </Container>
        }
    }
}
