use gloo::console;
use js_sys::Date;
use shared_lib::utils::structs::Task;
use yew::{function_component, html, Component, Context, Html};

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
                <div>
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
            <p class="bg-red-155">{"This string means there is multiple components and there is tailwindcss support."}</p>
        }
    }
}
mod http_req_test {
    use shared_lib::utils::structs::{Test, Undefined};

    use super::*;

    pub(super) enum HttpMsg {
        Fetch(String),
        Tasks(Vec<serde_json::Value>),
        // Task(Task<Undefined>),
    }

    pub(super) struct HttpReq {
        // name: String,           // this will store the name of the person
        // age: usize,             // this will store the age of the person
        // tasks: Option<Vec<serde_json::Value>>, // this will store the person result
        // task: Option<Vec<serde_json::Value>>, // this will store the person result
        tests: Vec<Task<Test>>,
        undefined: Vec<Task<Undefined>>,
    }

    impl Component for HttpReq {
        type Message = HttpMsg;
        type Properties = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                tests: Vec::new(),
                undefined: Vec::new(),
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
            <div>
                <button class="btn btn-blue" onclick={ctx.link().callback(|_|{console::log!("sending request..."); HttpMsg::Fetch("http://127.0.0.1:8000/tasks".to_string())})}>{ "get tasks" }</button>
                <p>{ if self.tests.is_empty() { String::from("") } else {format!("{:?}", self.tests)}}</p>
                <p>{ if self.undefined.is_empty() { String::from("") } else { format!("{:?}", self.undefined) }}</p>
            </div>
            }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            let link = ctx.link().clone();

            match msg {
                HttpMsg::Fetch(url) => {
                    wasm_bindgen_futures::spawn_local(async move {
                        let tasks = get_tasks(url.as_str()).await;
                        link.send_message(HttpMsg::Tasks(tasks))
                    });

                    false
                }
                HttpMsg::Tasks(task) => {
                    // self.task = Some(task.clone());

                    task.into_iter().for_each(|task| {
                        if let Ok(task_test) = serde_json::from_value::<Task<Test>>(task.clone()) {
                            if !self.tests.contains(&task_test) {
                                self.tests.push(task_test)
                            }
                        }
                        if let Ok(task_undefined) = serde_json::from_value::<Task<Undefined>>(task)
                        {
                            if !self.undefined.contains(&task_undefined) {
                                self.undefined.push(task_undefined)
                            }
                        }
                    });
                    // console::log!("person has been set to: ", person);
                    true
                } // HttpMsg::Task(task) => {
                  //     self.task = Some(task.clone());
                  //     true
                  // }
            }
        }
    }

    async fn get_tasks(url: &str) -> Vec<serde_json::Value> {
        let resp = reqwest::get(url).await.unwrap();

        console::log!("response received");

        let tasks = resp.json::<Vec<serde_json::Value>>().await.unwrap();

        console::log!("json parsed");
        tasks
    }
}

#[function_component]
fn App() -> Html {
    html! {

        <div  class="pl-5">
            <counter::Counter/>
            <rawhtml::RawHtml/>
            <http_req_test::HttpReq/>
        </div>

    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
