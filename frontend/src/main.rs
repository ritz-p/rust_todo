mod app;
mod component;
mod http_client;
mod props;
use app::App;

fn main() {
    console_error_panic_hook::set_once();
    yew::Renderer::<App>::new().render();
}
