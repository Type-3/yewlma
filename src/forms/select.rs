use yew::prelude::*;
use yewtil::NeqAssign;
use yew_property_info::PropertyInfo;

#[derive(Debug, PartialEq, Clone)]
pub enum SelectFieldOption {
    Value(String),
    Labeled(String, String)
}

pub struct SelectField {
    link: ComponentLink<Self>,
    props: SelectFieldProps,
    selected: Vec<usize>
}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct SelectFieldProps {
    pub items: Vec<SelectFieldOption>,
    #[prop_or_default]
    pub multiple: bool,
}

pub enum Msg {}

impl Component for SelectField {
    type Message = Msg;
    type Properties = SelectFieldProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self { link, props, selected: vec![] }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let multiple = self.props.multiple.then_some("is-multiple");
        html! {
            <div class="field">
                <div class="control">
                  <div class=("select", multiple)>
                    <select multiple=self.props.multiple>
                        {
                            for self.props.items.iter().enumerate().map(|(index, item)| match item {
                                SelectFieldOption::Value(value) => {
                                    let active = self.selected.contains(&index).then_some("is-active");
                                    html! { <option value=value class=active>{value}</option>}
                                },
                                SelectFieldOption::Labeled(label, value) => {
                                    let active = self.selected.contains(&index).then_some("is-active");
                                    html! {<option value=value class=active>{label}</option>}
                                }
                            })
                        }
                    </select>
                  </div>
                </div>
            </div>
        }
    }
}
