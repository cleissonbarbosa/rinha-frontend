use components::root::App;

mod components;
mod services;
fn main() {
    yew::Renderer::<App>::new().render();
}
