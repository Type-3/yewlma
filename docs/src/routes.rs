use yew_route_breadcrumbs::BreadCrumbs;
use yew_router::prelude::*;
use yewlma::classes::Color;

#[derive(Debug, PartialEq, Clone, Switch, BreadCrumbs)]
pub enum AppRoutes {
    #[to = "/docs{*:rest}"]
    #[breadcrumbs]
    Docs(DocsRoutes),
    #[to = "/"]
    Index,
}

#[derive(Debug, PartialEq, Clone, Switch, BreadCrumbs)]
pub enum DocsRoutes {
    #[to = "/elements{*:rest}"]
    #[breadcrumbs]
    Elements(ElementsRoutes),
    #[to = "/components{*:rest}"]
    #[breadcrumbs]
    Components(ComponentsRoutes),
    #[to = "/forms{*:rest}"]
    #[breadcrumbs]
    Forms(FormsRoutes),
    #[to = "/layout{*:rest}"]
    #[breadcrumbs]
    Layout(LayoutRoutes),
}

#[derive(Debug, PartialEq, Clone, Switch, BreadCrumbs)]
#[breadcrumb("Components", route = "/docs/components/")]
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
    #[to = "/tabs"]
    #[breadcrumb("Tabs")]
    Tabs,
    #[to = "/"]
    Index,
}

#[derive(Debug, PartialEq, Clone, Switch, BreadCrumbs)]
#[breadcrumb("Layout", route = "/docs/layout/")]
pub enum LayoutRoutes {
    #[to = "/container"]
    #[breadcrumb("Container")]
    Container,
    #[to = "/columns"]
    #[breadcrumb("Columns")]
    Columns,
    #[to = "/column"]
    #[breadcrumb("Column")]
    Column,
    #[to = "/footer"]
    #[breadcrumb("Footer")]
    Footer,
    #[to = "/hero"]
    #[breadcrumb("Hero")]
    Hero,
    #[to = "/"]
    Index,
}

#[derive(Debug, PartialEq, Clone, Switch, BreadCrumbs)]
#[breadcrumb("Forms", route = "/docs/forms/")]
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
    Index,
}

#[derive(Debug, PartialEq, Clone, Switch, BreadCrumbs)]
#[breadcrumb("Elements", route = "/docs/elements/")]
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
            ElementsRoutes::Button => vec![
                ("colors", Color::Success, Color::White),
                ("sizes", Color::Warning, Color::Black),
            ],
            ElementsRoutes::Buttons => vec![
                ("colors", Color::Success, Color::White),
                ("sizes", Color::Warning, Color::Black),
            ],
            ElementsRoutes::Icons => vec![
                ("colors", Color::Success, Color::White),
                ("sizes", Color::Warning, Color::Black),
            ],
            ElementsRoutes::Tags => vec![
                ("colors", Color::Success, Color::White),
                ("sizes", Color::Warning, Color::Black),
            ],
            ElementsRoutes::Tag => vec![
                ("colors", Color::Success, Color::White),
                ("sizes", Color::Warning, Color::Black),
            ],
            ElementsRoutes::Notifications => vec![("colors", Color::Success, Color::White)],
            ElementsRoutes::Delete => vec![],
            ElementsRoutes::Index => vec![],
        }
    }
}
