use crate::prelude::*;
use paginator::{PageItem, Paginator};
use yew::prelude::*;
use yewtil::NeqAssign;
use yew_property_info::PropertyInfo;

pub struct Pagination {
    paginator: Paginator,
    link: ComponentLink<Self>,
    props: PaginationProps,
}

pub enum Msg {
    Next,
    Prev,
    ChangePage(usize),
}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct PaginationProps {
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub round: bool,
    pub total: i64,
    pub per_page: i64,
    pub page: i64,
    #[prop_or_default]
    pub on_page: Callback<usize>,
}

impl Component for Pagination {
    type Message = Msg;
    type Properties = PaginationProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let paginator = Paginator::builder(props.total as usize)
            .current_page(props.page as usize)
            .build_paginator()
            .expect("Failed to build paginator");
        Self {
            props,
            link,
            paginator,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.paginator = Paginator::builder(props.total as usize)
            .current_page(props.page as usize)
            .build_paginator()
            .expect("Failed to build paginator");
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::ChangePage(item) => self.props.on_page.emit(item),
            Msg::Next => self.props.on_page.emit(self.props.page as usize + 1),
            Msg::Prev => self.props.on_page.emit(self.props.page as usize - 1),
        }
        false
    }

    fn view(&self) -> Html {
        let size = self.props.size.is();
        let round = self.props.round.then_some("is-rounded");
        let items = self.paginator.paginate().into_iter();
        html! {
            <nav class=("pagination", size, round)>
              <ul class="pagination-list">
                {
                    for items.map(|item| match item {
                        PageItem::Prev(_page) => html_nested! {
                            <li><a class="pagination-previous has-background-white" onclick=self.link.callback(|_|Msg::Prev)>{"«"}</a></li>
                        },
                        PageItem::Page(page) => html_nested! {
                            <li><a class="pagination-link has-background-white" onclick=self.link.callback(move |_| Msg::ChangePage(page.get()))>{page}</a></li>
                        },
                        PageItem::CurrentPage(page) => html_nested! {
                            <li><a class="pagination-link is-current has-text-white">{page}</a></li>
                        },
                        PageItem::Ignore => html_nested! {
                            <li><span class="pagination-ellipsis has-background-white">{"…"}</span></li>
                        },
                        PageItem::Next(_page) => html_nested! {
                            <li><a class="pagination-next has-background-white" onclick=self.link.callback(|_| Msg::Next)>{"»"}</a></li>
                        },
                        _ => html_nested!{}
                    })
                }
              </ul>
            </nav>
        }
    }
}
