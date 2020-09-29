use std::marker::PhantomData;

use yew::prelude::*;
use yewtil::{Pure, PureComponent};
use yew_property_info::HasPropertyInfo;


pub type PropsTable<T> = Pure<PurePropsTable<T>>;

#[derive(Debug, PartialEq, Clone, Properties)]
pub struct PurePropsTable<T: HasPropertyInfo + Clone + PartialEq + 'static> {
    #[prop_or_default]
    _phantom_props: PhantomData<T>
}

impl <T: HasPropertyInfo + Clone + PartialEq + 'static>PureComponent for PurePropsTable<T> {
    fn render(&self) -> Html {
        html! {
            <table class="table is-stripped is-fullwidth">
              <thead>
                <th>{"Property"}</th><th>{"Required"}</th><th>{"Type"}</th><th>{"Default"}</th><th>{"Description"}</th>
              </thead>
              <tbody>
              {
                  for T::property_info().fields.iter().map(|item| {
                      html! {<tr><td>{item.name}</td>
                                        <td>{item.required}</td>
                                         <td>{item.ty}</td>
                                         <td>{item.default.unwrap_or_else(|| "None".into())}</td>
                                         <td>{item.description.unwrap_or_else(|| "None".into())}</td></tr>
                                }
                  })
              }
              </tbody>
            </table>
        }
    }
}
