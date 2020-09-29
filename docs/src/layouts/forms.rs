use yew::prelude::*;
use yewtil::NeqAssign;
use yewlma::prelude::*;
use yew_router::components::RouterAnchor;
use crate::pages::{InputPage, FormsIndex, CheckBoxPage, TextAreaPage};
use crate::routes::*;

pub struct FormsLayout {
    props: Props,
    _link: ComponentLink<Self>
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Props {
    pub route: FormsRoutes
}

impl Component for FormsLayout {
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
              <p class="menu-label">{"Forms"}</p>
              <ul class="menu-list">
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Forms(FormsRoutes::Input))>{"Input"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Forms(FormsRoutes::TextArea))>{"TextArea"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Forms(FormsRoutes::CheckBox))>{"CheckBox"}</RouterAnchor<AppRoutes>></li>
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
                    FormsRoutes::Index => html! {<FormsIndex />},
                    FormsRoutes::Input => html! { <InputPage /> },
                    FormsRoutes::TextArea => html !{ <TextAreaPage /> },
                    FormsRoutes::CheckBox => html! { <CheckBoxPage /> }
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
