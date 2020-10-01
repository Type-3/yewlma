mod index;
pub use self::index::IndexPage;

mod elements;
pub use self::elements::{
    ButtonPage, ButtonsPage, DeletePage, ElementsIndex, IconsPage, NotificationPage, TagPage,
    TagsPage,
};

mod components;
pub use self::components::{BreadCrumbsPage, ComponentsIndex, DropDownPage, PaginationPage};

mod forms;
pub use self::forms::{CheckBoxPage, FormsIndex, InputPage, TextAreaPage};

mod layout;
pub use self::layout::{ColumnPage, ColumnsPage, ContainerPage, LayoutsIndex};
