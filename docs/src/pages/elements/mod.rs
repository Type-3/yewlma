use yew::prelude::*;
use yew_router::components::RouterAnchor;
use yewlma::prelude::*;
use crate::routes::*;

mod tags;
pub use self::tags::TagsPage;
mod buttons;
pub use self::buttons::ButtonsPage;
mod button;
pub use self::button::ButtonPage;
mod notifications;
pub use self::notifications::NotificationPage;
mod delete;
pub use self::delete::DeletePage;
mod icon;
pub use self::icon::IconsPage;
mod tag;
pub use self::tag::TagPage;

pub struct ElementsIndex;

impl Component for ElementsIndex {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _link: ComponentLink<Self>) -> Self {
        ElementsIndex
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <div class="tile is-ancestor">
              <div class="tile is-vertical is-8">
                <div class="tile">
                  <div class="tile is-parent is-vertical">
                    <RouterAnchor<AppRoutes> classes="tile is-child notification is-primary" route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Delete))>
                      <p class="title">{"Delete"}</p>
                      <p class="subtitle">{"Delete button"}</p>
                    </RouterAnchor<AppRoutes>>
                    <RouterAnchor<AppRoutes> classes="tile is-child notification is-warning" route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Icons))>
                      <p class="title">{"Icons"}</p>
                      <p class="subtitle">{"Display font icons"}</p>
                    </RouterAnchor<AppRoutes>>
                  </div>
                  <div class="tile is-parent">
                    <RouterAnchor<AppRoutes> classes="tile is-child notification is-info" route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Tags))>
                      <p class="title">{"Tags"}</p>
                      <p class="subtitle">{"Informational Tags"}</p>
                      <figure class="image is-4by3">
                        <img src="https://bulma.io/images/placeholders/640x480.png" />
                      </figure>
                    </RouterAnchor<AppRoutes>>
                  </div>
                </div>
                <div class="tile is-parent">
                  <RouterAnchor<AppRoutes> classes="tile is-child notification is-light" route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Notifications))>
                    <p class="title">{"Notifications"}</p>
                    <p class="subtitle">{"Display important information"}</p>
                    <div class="content">
                         <Notification color=Color::Danger>{"Important message"}</Notification>
                    </div>
                  </RouterAnchor<AppRoutes>>
                </div>
              </div>
              <div class="tile is-parent">
                <RouterAnchor<AppRoutes> classes="tile is-child notification is-success" route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Buttons))>
                  <div class="content">
                    <p class="title">{"Buttons"}</p>
                    <p class="subtitle">{"The classic button element"}</p>
                    <div class="content">

                    </div>
                  </div>
                </RouterAnchor<AppRoutes>>
              </div>
            </div>
        }
    }
}
