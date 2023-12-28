use super::super::*;
use shared_lib::utils::structs::{Task, Test, Undefined};
use yew::Properties;

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
            task: ctx.props().task.clone(),
        }
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        html! {
            <>
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
            </>
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        true
    }
}
