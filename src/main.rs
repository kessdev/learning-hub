mod app;
mod views;
mod components;
mod models;
mod utils;

use app::App;

fn main() {
    yew::Renderer::<App>::new().render();
}
