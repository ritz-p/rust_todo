mod app;
mod component;
mod props;
mod http_client;
use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
