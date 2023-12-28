use super::super::*;
use super::task_component::TaskComponent;
use shared_lib::utils::structs::{TaskBuilder, Undefined};

pub(crate) enum TaskListMessage {
    HttpFetchTasks(String),
    UpdateTaskComponents(Vec<serde_json::Value>),
    AddTaskComponent(String),
}

pub(crate) struct TaskList {
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
        let task = TaskBuilder::<Undefined>::new();

        if self.task_components.is_empty() {
            html! {
            <div>
                // <super::task_form::Form onsignal={ctx.link().callback(|_| TaskListMessage::AddTaskComponent(task))}/>
                <button class="btn btn-blue" onclick={ctx.link().callback(|_|{console::log!("sending request..."); TaskListMessage::HttpFetchTasks("http://localhost:8000/tasks".to_string())})}>{ "get tasks" }</button>
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
        let link = ctx.link().clone();

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
            TaskListMessage::AddTaskComponent(value) => {
                println!("{}", value);
                false
            }
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
