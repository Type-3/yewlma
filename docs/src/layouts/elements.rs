use crate::pages::{
    ButtonPage, ButtonsPage, DeletePage, ElementsIndex, IconsPage, NotificationPage, TagPage,
    TagsPage,
};
use crate::routes::ElementsRoutes;
use crate::routes::*;
use yew::prelude::*;
use yew_router::components::RouterAnchor;
use yewlma::prelude::*;
use yewtil::NeqAssign;

pub struct ElementsLayout {
    props: Props,
    _link: ComponentLink<Self>,
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Props {
    pub route: ElementsRoutes,
}

impl Component for ElementsLayout {
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
              <p class="menu-label">{"Elements"}</p>
              <ul class="menu-list">
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Button))>{"Button"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Buttons))>{"Buttons"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Icons))>{"Icon"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Tag))>{"Tag"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Tags))>{"Tags"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Icons))>{"Icons"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Delete))>{"Delete"}</RouterAnchor<AppRoutes>></li>
                <li><RouterAnchor<AppRoutes> route=AppRoutes::Docs(DocsRoutes::Elements(ElementsRoutes::Notifications))>{"Notifications"}</RouterAnchor<AppRoutes>></li>
              </ul>
            </aside>
            </Column>
            <Column>
            <Container class="mt-2">
            <Columns class="is-vcentered">
            <Column><BreadCrumbs crumbs=self.props.route.breadcrumbs() size=Size::Medium /></Column>
            <div class="field is-grouped is-grouped-multiline mr-3">
            {
                for self.props.route.get_route_tags().iter().map(|tag| {
                    html_nested!{<div class="control"><Tags addons=true><Tag>{tag.0}</Tag><Tag color={tag.1}><Icon name="fa fa-check-circle" color={tag.2} /></Tag></Tags></div>}
                })
            }
            </div>
            </Columns>
            <Columns>
            <Column>
            {
                match self.props.route {
                    ElementsRoutes::Index => html! {<ElementsIndex />},
                    ElementsRoutes::Icons => html! { <IconsPage /> },
                    ElementsRoutes::Button => html! { <ButtonPage /> },
                    ElementsRoutes::Buttons => html! { <ButtonsPage /> },
                    ElementsRoutes::Tag => html! { <TagPage /> },
                    ElementsRoutes::Tags => html! { <TagsPage /> },
                    ElementsRoutes::Delete => html! { <DeletePage /> },
                    ElementsRoutes::Notifications => html! { <NotificationPage /> }
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
