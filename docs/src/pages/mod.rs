mod index;
pub use self::index::IndexPage;

mod elements;
pub use self::elements::{
    ButtonPage, ButtonsPage, DeletePage, ElementsIndex, IconsPage, NotificationPage, TagPage,
    TagsPage,
};

mod components;
pub use self::components::{TabsPage, BreadCrumbsPage, ComponentsIndex, DropDownPage, PaginationPage};

mod forms;
pub use self::forms::{CheckBoxPage, FormsIndex, InputPage, TextAreaPage, FilePage};

mod layout;
pub use self::layout::{ColumnPage, FooterPage, HeroPage, ColumnsPage, ContainerPage, LayoutsIndex};
