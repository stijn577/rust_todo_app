use gloo::console;
use js_sys::Date;
use yew::{function_component, html, Component, Context, Html};
use yew_rocket::utils::structs::Person;

mod counter {
    use super::*;

    pub(super) enum CounterMsg {
        Increment,
        Decrement,
    }

    pub(super) struct Counter {
        value: i64, // This will store the counter value
    }

    impl Component for Counter {
        type Message = CounterMsg;
        type Properties = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self { value: 0 }
        }

        fn update(&mut self, _ctx: &Context<Self>, msg: Self::Message) -> bool {
            match msg {
                CounterMsg::Increment => {
                    self.value += 1;
                    console::log!("plus one"); // Will output a string to the browser console
                    true // Return true to cause the displayed change to update
                }
                CounterMsg::Decrement => {
                    self.value -= 1;
                    console::log!("minus one");
                    true
                }
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <div class="pl-5">
                        // A button to send the Increment message
                        <button class="btn btn-green" onclick={ctx.link().callback(|_| CounterMsg::Increment)}>
                            { "+1" }
                        </button>

                        // A button to send the Decrement message
                        <button class="btn btn-red" onclick={ctx.link().callback(|_| CounterMsg::Decrement)}>
                            { "-1" }
                        </button>

                        // A button to send two Increment messages
                        <button class="btn btn-blue" onclick={ctx.link().batch_callback(|_| vec![CounterMsg::Increment, CounterMsg::Increment])}>
                            { "+2" }
                        </button>

                    // Display the current value of the counter
                    <p class="counter">
                        { self.value }
                    </p>

                    // Display the current date and time the page was rendered
                    <p class="footer">
                        { "Rendered: " }
                        { String::from(Date::new_0().to_string()) }
                    </p>
                </div>
            }
        }
    }
}
mod rawhtml {
    use super::*;

    #[function_component(RawHtml)]
    pub(super) fn raw() -> Html {
        html! {
            <p class="bg-red-100 pl-5">{"This string means there is multiple components and there is tailwindcss support."}</p>
        }
    }
}
mod http_req_test {
    use super::*;

    pub(super) enum HttpMsg {
        Fetch(&'static str),
        Person(Person),
    }

    pub(super) struct HttpReq {
        data: Option<Person>, // this will store the person result
    }

    impl Component for HttpReq {
        type Message = HttpMsg;
        type Properties = ();

        fn create(ctx: &Context<Self>) -> Self {
            Self { data: None }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
            <div>
                <button class="btn btn-blue" onclick={ctx.link().callback(|_|{console::log!("send request"); HttpMsg::Fetch("http://127.0.0.1:8000/hello/stijn/19")})}>{ "get stijn" }</button>
                <p>{ if let Some(data) = &self.data {format!("{:?}", data) } else { "".to_string() }}</p>
            </div>
            }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            let link = ctx
                .link()
                .clone();

            match msg {
                HttpMsg::Fetch(url) => {
                    wasm_bindgen_futures::spawn_local(async move {
                        let resp = reqwest::get(url)
                            .await
                            .unwrap();

                        console::log!("response received");

                        let person = resp
                            .json::<Person>()
                            .await
                            .unwrap();

                        console::log!("json parsed");

                        link.send_message(HttpMsg::Person(person))
                    });

                    false
                }
                HttpMsg::Person(person) => {
                    self.data = Some(person);
                    true
                }
            }
        }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div>
            <counter::Counter/>
            <rawhtml::RawHtml/>
            <http_req_test::HttpReq/>
        </div>
    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
