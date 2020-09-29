use yewlma::prelude::*;
use yew::prelude::*;

use crate::routes::*;
use yewlma::components::navbar::*;

pub struct DemoNavBar ;

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
            <NavBar shadow=true color=Color::Primary>
              <Brand<AppRoutes> route=AppRoutes::Index>
                <h1 class="navbar-item title has-text-white">{"Yewlma"}</h1>
              </Brand<AppRoutes>>

              <div id="demoNavBar" class="navbar-menu">
                <div class="navbar-end">
                <div class="navbar-item has-dropdown is-hoverable">
                  <Link>{"Documentation"}</Link>
                  <div class="navbar-dropdown">
                    <Item<AppRoutes> route=AppRoutes::Elements(ElementsRoutes::Index)>
                      {"Elements"}
                    </Item<AppRoutes>>
                    <Item<AppRoutes> route=AppRoutes::Components(ComponentsRoutes::Index)>
                      {"Components"}
                    </Item<AppRoutes>>
                    <Item<AppRoutes> route=AppRoutes::Forms(FormsRoutes::Index)>
                      {"Forms"}
                    </Item<AppRoutes>>
                  </div>
                </div>
                </div>
              </div>
            </NavBar>
        }
    }
}
