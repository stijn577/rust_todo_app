use gloo::console;
use js_sys::Date;
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
            <p class="bg-red-100">{"If this string's background is red, it means there is multiple components and there is tailwindcss support."}</p>
        }
    }
}
mod task_list {
    use super::task_component::TaskComponent;

    use super::*;

    pub(super) enum TaskListMessage {
        HttpFetchTasks(String),
        UpdateTaskComponents(Vec<serde_json::Value>),
        // Task(Task<Undefined>),
    }

    pub(super) struct TaskList {
        task_components: Vec<serde_json::Value>,
    }

    impl Component for TaskList {
        type Message = TaskListMessage;
        type Properties = ();

        fn create(_ctx: &Context<Self>) -> Self {
            Self {
                // tests: Vec::new(),
                // undefined: Vec::new(),
                task_components: Vec::new(),
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            if self
                .task_components
                .is_empty()
            {
                html! {
                <div>
                    <button class="btn btn-blue" onclick={ctx.link().callback(|_|{console::log!("sending request..."); TaskListMessage::HttpFetchTasks("http://localhost:8000/tasks".to_string())})}>{ "get tasks" }</button>
                    // <p>{ if self.tests.is_empty() { String::from("") } else {format!("{:?}", self.tests)}}</p>
                    // <p>{ if self.undefined.is_empty() { String::from("") } else { format!("{:?}", self.undefined) }}</p>
                </div>
                }
            } else {
                html! {
                    <div>{ 
                        self.task_components
                        .clone()
                        .into_iter()
                        .map(|task_component| html! { <TaskComponent task={task_component}/> }).collect::<Html>() 
                    }
                    </div>

                }
            }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            let link = ctx
                .link()
                .clone();

            match msg {
                TaskListMessage::HttpFetchTasks(url) => {
                    wasm_bindgen_futures::spawn_local(async move {
                        let tasks = get_tasks(url.as_str()).await;
                        link.send_message(TaskListMessage::UpdateTaskComponents(tasks))
                    });

                    false
                }
                TaskListMessage::UpdateTaskComponents(tasks) => {
                    self.task_components = tasks;

                    true
                }
            }
        }
    }

    async fn get_tasks(url: &str) -> Vec<serde_json::Value> {
        let resp = reqwest::get(url)
            .await
            .unwrap();

        console::log!("response received");

        let tasks = resp
            .json::<Vec<serde_json::Value>>()
            .await
            .unwrap();

        console::log!("json parsed");
        tasks
    }
}

mod task_component {
    use shared_lib::utils::structs::{Task, Test, Undefined};
    use yew::Properties;

    use super::*;

    pub(super) enum TaskMsg {}

    #[derive(Debug, Clone)]
    pub(super) struct TaskComponent {
        pub(crate) task: serde_json::Value,
    }

    #[derive(Properties, PartialEq)]
    pub(super) struct TaskProps {
        pub(super) task: serde_json::Value,
    }

    impl Component for TaskComponent {
        type Message = TaskMsg;

        type Properties = TaskProps;

        fn create(ctx: &Context<Self>) -> Self {
            Self {
                task: ctx
                    .props()
                    .task
                    .clone(),
            }
        }

        fn view(&self, ctx: &Context<Self>) -> Html {
            html! {
                <div>
                    <p> {
                        if let Ok(task) = serde_json::from_value::<Task<Undefined>>(self.task.clone()){
                            format!("{:#?}", task.title())
                        } else if let Ok(task) = serde_json::from_value::<Task<Test>>(self.task.clone()){
                            format!("{:#?}", task.title())
                        } else {
                            String::new()
                        }
                    }
                    </p>
                </div>
            }
        }

        fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
            true
        }
    }
}

#[function_component]
fn App() -> Html {
    html! {
        <div class="pl-5">
            <counter::Counter/>
            <rawhtml::RawHtml/>
            <task_list::TaskList/>
        </div>

    }
}
fn main() {
    yew::Renderer::<App>::new().render();
}
