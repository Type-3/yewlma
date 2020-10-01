use crate::routes::*;
use yew::prelude::*;
use yew_router::prelude::*;

mod input;
pub use self::input::InputPage;
mod textarea;
pub use self::textarea::TextAreaPage;
mod checkbox;
pub use self::checkbox::CheckBoxPage;

pub struct FormsIndex;

impl Component for FormsIndex {
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
            <div>
            <span class="title">{"TODO: Make index page"}</span>
            <ul>
                 <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Forms(FormsRoutes::Input))>{"Input"}</RouterAnchor<AppRoutes>></li>
                 <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Forms(FormsRoutes::TextArea))>{"TextArea"}</RouterAnchor<AppRoutes>></li>
                 <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Forms(FormsRoutes::CheckBox))>{"CheckBox"}</RouterAnchor<AppRoutes>></li>
            </ul>
            </div>
        }
    }
}
