use yew_router::prelude::*;
use yew_route_breadcrumbs::BreadCrumbs;
use yewlma::classes::Color;

#[derive(Debug, PartialEq, Clone, Switch, BreadCrumbs)]
pub enum AppRoutes {
    #[to = "/elements{*:rest}"]
    #[breadcrumbs]
    Elements(ElementsRoutes),
    #[to = "/components{*:rest}"]
    #[breadcrumbs]
    Components(ComponentsRoutes),
    #[to = "/forms{*:rest}"]
    #[breadcrumbs]
    Forms(FormsRoutes),
    #[to = "/"]
    Index,
}

#[derive(Debug, PartialEq, Clone, Switch,  BreadCrumbs)]
#[breadcrumb("Components", route = "/components/")]
pub enum ComponentsRoutes {
    #[to = "/dropdown"]
    #[breadcrumb("DropDown")]
    DropDown,
    #[to = "/breadcrumbs"]
    #[breadcrumb("BreadCrumbs")]
    BreadCrumbs,
    #[to = "/pagination"]
    #[breadcrumb("Pagination")]
    Pagination,
    #[to = "/"]
    Index,
}

#[derive(Debug, PartialEq, Clone, Switch, BreadCrumbs)]
#[breadcrumb("Forms", route = "/forms/")]
pub enum FormsRoutes {
    #[to = "/input"]
    #[breadcrumb("Input")]
    Input,
    #[to = "/textarea"]
    #[breadcrumb("TextArea")]
    TextArea,
    #[to = "/checkbox"]
    #[breadcrumb("CheckBox")]
    CheckBox,
    #[to = "/"]
    Index
}

#[derive(Debug, PartialEq, Clone, Switch, BreadCrumbs)]
#[breadcrumb("Elements", route="/elements/")]
pub enum ElementsRoutes {
    #[to = "/buttons"]
    #[breadcrumb("Buttons")]
    Buttons,
    #[to = "/button"]
    #[breadcrumb("Button")]
    Button,
    #[to = "/notifications"]
    #[breadcrumb("Notification")]
    Notifications,
    #[to = "/delete"]
    #[breadcrumb("Delete")]
    Delete,
    #[to = "/icons"]
    #[breadcrumb("Icons")]
    Icons,
    #[to = "/tags"]
    #[breadcrumb("Tags")]
    Tags,
    #[to = "/tag"]
    #[breadcrumb("Tag")]
    Tag,
    #[to = "/"]
    Index,
}

impl ElementsRoutes {
    pub fn get_route_tags(&self) -> Vec<(&'static str, Color, Color)> {
        match self {
            ElementsRoutes::Button => vec![("colors", Color::Success, Color::White), ("sizes", Color::Warning, Color::Black)],
            ElementsRoutes::Buttons => vec![("colors", Color::Success, Color::White), ("sizes", Color::Warning, Color::Black)],
            ElementsRoutes::Icons => vec![("colors", Color::Success, Color::White), ("sizes", Color::Warning, Color::Black)],
            ElementsRoutes::Tags => vec![("colors", Color::Success, Color::White), ("sizes", Color::Warning, Color::Black)],
            ElementsRoutes::Tag => vec![("colors", Color::Success, Color::White), ("sizes", Color::Warning, Color::Black)],
            ElementsRoutes::Notifications => vec![("colors", Color::Success, Color::White)],
            ElementsRoutes::Delete => vec![],
            ElementsRoutes::Index => vec![]
        }
    }
}
