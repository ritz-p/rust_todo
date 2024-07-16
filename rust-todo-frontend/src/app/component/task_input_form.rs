use web_sys::HtmlInputElement;
use yew::prelude::*;

pub struct TaskInputForm {
    task_name: String,
}

pub enum Msg {
    UpdateTaskName(String),
    Submit,
}

impl Component for TaskInputForm {
    type Message = Msg;
    type Properties = ();

    fn create(_: &Context<Self>) -> Self {
        Self {
            task_name: String::new(),
        }
    }

    fn update(&mut self, ctx: &Context<Self>, msg: Self::Message) -> bool {
        match msg {
            Msg::UpdateTaskName(name) => self.task_name = name,
            Msg::Submit => {
                web_sys::console::log_1(&format!("Task Name: {}", self.task_name).into());
            }
        }
        true
    }

    fn view(&self, ctx: &Context<Self>) -> Html {
        let oninput_task_name = ctx.link().callback(|e: InputEvent|{
            let input: HtmlInputElement = e.target_unchecked_into();
            Msg::UpdateTaskName(input.value())
        });
        html! {
            <form onsubmit={ctx.link().callback(|e: SubmitEvent| {
                e.prevent_default();
                Msg::Submit
            })}>
                <div>
                    <label for="task_name">{"Task Name: "}</label>
                    <input
                        type="text"
                        id="task_name"
                        value={self.task_name.clone()}
                        oninput={oninput_task_name}
                    />
                </div>
                <button type="submit">{"Submit"}</button>
            </form>
        }
    }
}
