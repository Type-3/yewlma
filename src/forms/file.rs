use yew::prelude::*;
use yewtil::NeqAssign;
use yew_property_info::PropertyInfo;

pub struct FileField {
    value: String,
    props: FileFieldProps,
    link: ComponentLink<Self>
}

#[derive(Debug, PartialEq, Clone, Properties, PropertyInfo)]
pub struct FileFieldProps {
    #[prop_or_default]
    pub value: Option<String>,
    #[prop_or_default]
    pub onchange: Callback<String>
}

pub enum Msg {
    Changed(ChangeData)
}

impl Component for FileField {
    type Message = Msg;
    type Properties = FileFieldProps;

    fn create(props: Self::Properties, link: ComponentLink<Self>) -> Self {
        let value = props.value.clone().unwrap_or_else(String::new);
        Self { props, link, value }
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        if let Some(ref value) = props.value {
            self.value = value.clone();
        }
        self.props.neq_assign(props)
    }

    fn update(&mut self, msg: Self::Message) -> ShouldRender {
        match msg {
            Msg::Changed(ChangeData::Files(file)) => {
                self.value = format!("{}", file.get(0).unwrap().name());
                self.props.onchange.emit(self.value.clone());
            },
            _ => {}
        }
        true
    }

    fn view(&self) -> Html {
        html! {
           <div class="file has-name">
             <label class="file-label">
               <input class="file-input" type="file" name="resume" onchange=self.link.callback(Msg::Changed) />
               <span class="file-cta">
                 <span class="file-icon">
                   <i class="fa fa-upload"></i>
                 </span>
                 <span class="file-label">
                   {"Choose a file…"}
                 </span>
               </span>
               <span class="file-name">
                 {&self.value}
               </span>
             </label>
           </div>
        }
    }
}
