// use super::*;
use super::super::*;

pub enum CounterMsg {
    Increment,
    Decrement,
}

pub(crate) struct Counter {
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
