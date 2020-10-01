use yew::prelude::*;
use yew_router::components::RouterAnchor;
use yewlma::prelude::*;

use crate::routes::*;

pub struct DemoNavBar;

impl Component for DemoNavBar {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        DemoNavBar
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <nav class="navbar">
              <div class="container">
                <div class="navbar-brand">
                  <RouterAnchor<AppRoutes> classes="title navbar-item" route=AppRoutes::Index>{"Yewlma"}</RouterAnchor<AppRoutes>>
                  <span class="navbar-burger burger" data-target="navbarMenuHeroA">
                    <span></span>
                    <span></span>
                    <span></span>
                  </span>
                </div>
                <div id="navbarMenuHeroA" class="navbar-menu">
                  <div class="navbar-end">
                  <RouterAnchor<AppRoutes> classes="navbar-item is-active" route=AppRoutes::Docs(DocsRoutes::Layout(LayoutRoutes::Index))>
                    {"Layout"}
                  </RouterAnchor<AppRoutes>>
                    <RouterAnchor<AppRoutes> classes="navbar-item is-active" route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Index))>
                      {"Elements"}
                    </RouterAnchor<AppRoutes>>
                    <RouterAnchor<AppRoutes> classes="navbar-item" route=AppRoutes::Docs(DocsRoutes::Components(ComponentsRoutes::Index))>
                       {"Components"}
                    </RouterAnchor<AppRoutes>>
                    <RouterAnchor<AppRoutes> classes="navbar-item" route=AppRoutes::Docs(DocsRoutes::Forms(FormsRoutes::Index))>
                    {"Forms"}
                    </RouterAnchor<AppRoutes>>
                    <a  href="https://github.com/Type-3/yewlma" class="navbar-item">
                        <Icon name="fa fa-github" />
                    </a>
                  </div>
                </div>
              </div>
            </nav>
        }
    }
}
