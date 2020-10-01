use yew::prelude::*;
use yewlma::prelude::*;
use yewtil::NeqAssign;
use yew_property_info::HasPropertyInfo;
use crate::components::{DemoContainer, PropsTable};
use std::marker::PhantomData;

pub struct PropertyInformation<T: HasPropertyInfo>(Props, PhantomData<T>);

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct Props {

}

impl <T: HasPropertyInfo + Clone + PartialEq + 'static>Component for PropertyInformation<T> {
    type Message = ();
    type Properties = Props;

    fn create(props: Self::Properties, _: ComponentLink<Self>) -> Self {
        PropertyInformation(props, Default::default())
    }

    fn change(&mut self, props: Self::Properties) -> ShouldRender {
        self.0.neq_assign(props)
    }

    fn update(&mut self, _: Self::Message) -> ShouldRender {
        false
    }

    fn view(&self) -> Html {
        let prop_info = T::property_info();
        html! {
            <Columns>
            <Column>
               <h1 class="title">{"Properties"}</h1>
               <h2 class="subtitle">{prop_info.module}{"::"}{prop_info.ty}</h2>
               <DemoContainer>
                   <PropsTable<T> />
               </DemoContainer>
             </Column>
            </Columns>
        }
    }
}
