use yew::prelude::*;
use yew_property_info::PropertyInfo;
use yewtil::NeqAssign;

pub struct CheckBoxField {
    props: CheckBoxFieldProps,
    link: ComponentLink<Self>,
    value: bool,
}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct CheckBoxFieldProps {
    #[prop_or_default]
    pub value: bool,
    #[prop_or_default]
    pub label: Option<String>,
    #[prop_or_default]
    pub on_toggle: Callback<bool>,
}

pub struct Clicked(yew::ChangeData);

impl Component for CheckBoxField {
    type Message = Clicked;
    type Properties = CheckBoxFieldProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        Self {
            link,
            value: props.value.clone(),
            props,
        }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.value = props.value;
        self.props.neq_assign(props)
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        self.value = !self.value;
        self.props.on_toggle.emit(self.value);
        true
    }

    fn view(&self) -> Html {
        let input_margin = self.props.label.as_ref().map(|_| "mr-2");
        html! {
            <label class="checkbox">
                <input class=input_margin type="checkbox" checked=self.value onchange=self.link.callback(Clicked) />
                {
                    if let Some(label) = &self.props.label {
                        html! { &label }
                    } else {
                        html! {}
                    }
                }
            </label>
        }
    }
}
