use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;

use crate::{components::title_bar::TitleBar, views::chat::navigation::nav_panel::NavPanel};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"])]
    async fn invoke(cmd: &str, args: JsValue) -> JsValue;
}

#[derive(Serialize, Deserialize)]
struct GreetArgs<'a> {
    name: &'a str,
}

#[function_component(App)]
pub fn app() -> Html {
    html! {
        <main class="w-screen h-screen flex flex-col">
            <TitleBar />
            <div class="grow flex flex-row">
                <NavPanel />
            </div>
        </main>
    }
}
