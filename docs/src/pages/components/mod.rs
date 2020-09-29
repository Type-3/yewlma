use yew::prelude::*;

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
            <p>{"Components Index Page"}</p>
        }
    }
}
