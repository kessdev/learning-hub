use serde::{Deserialize, Serialize};
use wasm_bindgen::prelude::*;
use yew::prelude::*;
use bounce::BounceRoot;

use crate::{components::titlebar::Titlebar, views::chat::chat_view::ChatView};

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
        <BounceRoot>
            <main class="w-screen h-screen flex flex-col">
                <Titlebar />
                <ChatView />
            </main>
        </BounceRoot>
    }
}
