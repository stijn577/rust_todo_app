use gloo::console;
use js_sys::Date;
use yew::{function_component, html, Component, Context, Html};

mod components;

#[function_component]
fn App() -> Html {
    asdf html! {
        <>
            // <components::counter::Counter/>
            <components::rawhtml::RawHtml/>
            // <components::task_form::Form/>
            <components::task_list::TaskList/>
        </>

    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
