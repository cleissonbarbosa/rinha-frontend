extern crate base64;
use components::app::App;

mod components;
mod services;
fn main() {
    yew::Renderer::<App>::new().render();
}
