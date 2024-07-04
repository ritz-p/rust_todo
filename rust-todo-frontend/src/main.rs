use gloo::console;
use yew::prelude::*;

#[function_component]
fn App() -> Html {
    console::log!("Hello log!");
    console::debug!("Hello debug!");
    console::info!("Hello info!");
    console::warn!("Hello warn!");
    console::assert!(true, "Hello world!");
    console::assert!(false, "This will fail!");
    console::table!([0, 1, 2], ["a", "b"]);
    html! {
        <div>
            {"hello yew"}
        </div>
    }
}

fn main() {
    yew::Renderer::<App>::new().render();
}
