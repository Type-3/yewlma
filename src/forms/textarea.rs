use crate::prelude::*;
use yew::prelude::*;
use yew_property_info::PropertyInfo;
use yewtil::NeqAssign;

pub enum Msg {
    Changed(ChangeData),
}

pub struct TextAreaField {
    props: TextAreaFieldProps,
    link: ComponentLink<Self>,
}

#[derive(Debug, Clone, PartialEq, Properties, PropertyInfo)]
pub struct TextAreaFieldProps {
    #[prop_or_default]
    pub name: Option<String>,
    #[prop_or("text".into())]
    pub ty: String,
    #[prop_or_default]
    pub error: Option<String>,
    #[prop_or_default]
    pub class: Option<String>,
    #[prop_or_default]
    pub color: Option<Color>,
    #[prop_or_default]
    pub size: Option<Size>,
    #[prop_or_default]
    pub placeholder: Option<String>,
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub disabled: bool,
    #[prop_or_default]
    pub loading: bool,
    #[prop_or_default]
    pub rounded: bool,
    #[prop_or_default]
    pub onchange: Callback<String>,
    #[prop_or_default]
    pub label: Option<String>,
}

impl Component for TextAreaField {
    type Message = Msg;
    type Properties = TextAreaFieldProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        TextAreaField { props, link }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        if let Msg::Changed(ChangeData::Value(data)) = msg {
            self.props.onchange.emit(data);
        }
        false
    }

    fn view(&self) -> Html {
        let size = self.props.size.is();
        let mut color = self.props.color.is();
        let loading = self.props.loading.then_some("is-loading");
        let round = self.props.rounded.then_some("is-rounded");
        let label = self
            .props
            .label
            .as_ref()
            .map(|item| html_nested! { <label>{item}</label> })
            .unwrap_or_else(|| html! {});
        let error = self
            .props
            .error
            .clone()
            .map(|item| html! { <p class="help is-danger has-text-left">{item.clone()}</p> });
        if error.is_some() {
            color = "is-danger".into();
        }
        let value = self.props.value.clone().unwrap_or_else(String::new);
        html! {
            <div class="field">
              {label}
              <div class=("control", loading)>
                {
                    if let Some(error) = error {
                        error.clone()
                    } else {
                        html_nested! {}
                    }
                }
                <textarea name?=self.props.name.as_ref()
                       type=&self.props.ty
                       value=value
                       placeholder?=self.props.placeholder.as_ref()
                       onchange=self.link.callback(Msg::Changed)
                       disabled=self.props.disabled
                       class=("textarea", color, round, size, &self.props.class)/>
              </div>
            </div>
        }
    }
}
