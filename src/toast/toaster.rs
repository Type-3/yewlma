use crate::prelude::*;
use yew::prelude::*;
use yew::services::Task;
use yewtil::NeqAssign;

use std::collections::HashMap;
use std::time::Duration;

use super::*;

pub struct Toaster {
    counter: usize,
    props: Props,
    toasts: Vec<(usize, Toast)>,
    link: ComponentLink<Self>,
    jobs: HashMap<usize, Box<dyn Task>>,
    _producer: Box<dyn Bridge<ToastAgent>>,
}

pub enum Msg {
    New(Toast),
    Remove(usize),
    Timeout(usize),
}

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Props {
    #[prop_or_default]
    pub left: bool,
    #[prop_or_default]
    pub right: bool,
}

impl Component for Toaster {
    type Message = Msg;
    type Properties = Props;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let callback = link.callback(Msg::New);
        let _producer = ToastAgent::bridge(callback);
        let jobs = HashMap::new();
        Toaster {
            props,
            link,
            _producer,
            toasts: vec![],
            jobs,
            counter: 0,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::New(toast) => {
                let timeout = toast.timeout.unwrap_or(5);
                let id = self.counter;
                self.toasts.push((id, toast));
                let callback = self.link.callback(move |_| Msg::Timeout(id));
                let handle = yew::services::TimeoutService::spawn(
                    Duration::from_secs(timeout as u64),
                    callback,
                );
                self.jobs.insert(id, Box::new(handle));
                self.counter += 1;
            }
            Msg::Remove(req_id) => {
                self.toasts.retain(|(id, _)| id != &req_id);
                self.jobs.remove(&req_id);
            }
            Msg::Timeout(req_id) => {
                self.toasts.retain(|(id, _)| id != &req_id);
                self.jobs.remove(&req_id);
            }
        }
        true
    }

    fn view(&self) -> Html {
        let offset = if self.props.right {
            Some(ColSize::Eight)
        } else if self.props.left {
            None
        } else {
            Some(ColSize::Four)
        };
        html! {
            <div style="z-index:1000;position:absolute;width:100%;pointer-events:none">
            <Columns>
            <Column size=ColSize::Four offset=offset class="notices">
                {
                    for self.toasts.iter().map(|(dex, item)| {
                        let color_class = item.color.is();
                        let text_color = item.color.has_text();
                        let dex = dex.clone();
                        html_nested! {
                            <div class=("message", "my-5 mx-5", "is-small", color_class)>
                              <div class=("message-body", text_color, "px-2", "py-2")>
                                <p>{&item.message}
                                  <Delete color=item.color onsignal=self.link.callback(move |_| Msg::Remove(dex)) class="is-pulled-right" />
                                </p>
                              </div>
                            </div>
                        }
                    })
                }
            </Column>
            </Columns>
            </div>
        }
    }
}
