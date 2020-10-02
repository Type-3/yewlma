use crate::routes::*;
use yew::prelude::*;
use yew_router::components::RouterAnchor;
use yewlma::prelude::*;
pub struct IndexPage;

impl Component for IndexPage {
    type Message = ();
    type Properties = ();

    fn create(_: Self::Properties, _: ComponentLink<Self>) -> Self {
        IndexPage
    }

    fn change(&mut self, _: Self::Properties) -> ShouldRender {
        false
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        html! {
            <Hero color=Color::Primary bold=true fullheight=true>
            <HeroHead>
              <nav class="navbar">
                <div class="container">
                  <div class="navbar-brand">
                    <span class="title navbar-item">{"Yewlma"}</span>
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
            </HeroHead>
                <h1 class="title">
                  {"Bulma Components for Yew"}
                </h1>
                <h2 class="subtitle">
                  {"Subtitle"}
                </h2>
            </Hero>
        }
    }
}
