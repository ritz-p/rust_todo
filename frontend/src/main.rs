pub mod app;
pub mod component;
pub mod props;
pub mod utils;

use app::App;
fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
