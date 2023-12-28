// use yew::events::FocusEvent;
// use yew::events::InputData;
// use yew::prelude::*;
//
// pub struct FormFields {
//     name: String,
//     email: String,
// }
//
// #[derive(Properties, Clone, PartialEq)]
// pub struct FormProps {
//     pub onsubmit: Callback<FormFields>,
// }
//
// pub struct FormComponent {}
//
// pub enum Msg {
//     UpdateName(String),
//     UpdateEmail(String),
//     Submit,
// }
//
// impl Component for FormComponent {
//     type Message = Msg;
//     type Properties = FormProps;
//
//     fn create(ctx: &Context<Self>) -> Self {
//         Self {
//             link: ctx.props().link.clone(),
//             fields: ctx.props().fields.clone(),
//             props: ctx.props().props.clone(),
//         }
//     }
//
//     fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
//         match msg {
//             Msg::UpdateName(val) => {
//                 self.fields.name = val;
//                 true
//             }
//             Msg::UpdateEmail(val) => {
//                 self.fields.email = val;
//                 true
//             }
//             Msg::Submit => {
//                 self.props.onsubmit.emit(self.fields);
//                 false
//             }
//         }
//     }
//
//     fn view(&self, _ctx: &Context<Self>) -> Html {
//         let onsubmit = _ctx.clone().link.callback(|e: FocusEvent| {
//             e.prevent_default();
//             Msg::Submit
//         });
//
//         html! {
//              <form onsubmit={onsubmit}>
//                 <label>{"Name:"}</label>
//                 <input
//                      type="text"
//                      value={self.fields.name}
//                      oninput={self.link.callback(|e: InputData| Msg::UpdateName(e.value))}
//                 />
//                 <label>{"Email:"}</label>
//                 <input
//                    type="text"
//                    value={self.fields.email}
//                    oninput={self.link.callback(|e: InputData| Msg::UpdateEmail(e.value))}
//                 />
//                 <button type="submit">{"Submit"}</button>
//             </form>
//         }
//     }
// }

use shared_lib::utils::structs::{TaskBuilder, Undefined};
use yew::{html, prelude::*, Callback, Component};

enum FormMsg {
    UpdateName(String),
}

pub struct Form {
    onsignal: Callback<TaskBuilder<Undefined>>,
    // taskbuilder: TaskBuilder<Undefined>,
    name: String,
}

impl Component for Form {
    type Message = FormMsg;

    type Properties = ();

    fn create(ctx: &yew::prelude::Context<Self>) -> Self {
        todo!()
    }

    fn view(&self, ctx: &yew::prelude::Context<Self>) -> yew::prelude::Html {
        html! {
        <div>
            <p>{ self.name }</p>
            <input onchange={ctx.link().callback(|e: InputEvent| FormMsg::UpdateName(e.data().unwrap_or(String::new())))} value={self.name} />
            // <button onsubmit={}>{ "Submit" }</button>
        </div>
        }
    }

    fn update(&mut self, ctx: &yew::prelude::Context<Self>, msg: Self::Message) -> bool {
        match msg {
            FormMsg::UpdateName(name) => {
                self.name = name;
                true
            }
        }
    }
}
