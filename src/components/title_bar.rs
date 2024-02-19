use wasm_bindgen::prelude::*;
use serde_wasm_bindgen::to_value;
use wasm_bindgen_futures::spawn_local;
use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html};

use crate::components::image::{MinimizeImage, MaximizeImage, CloseImage};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[function_component(TitleBar)]
pub fn title_bar() -> Html {
    let on_minimize = Callback::from(move |_event: MouseEvent| {
        spawn_local(async move {
            let empty_args = to_value(&{}).unwrap();
            let response = invoke("minimize", empty_args).await;
            if response.is_err() {
                web_sys::console::error_1(&response.unwrap_err());
            }
        });
    });

    let on_maximize = Callback::from(move |_event: MouseEvent| {
        spawn_local(async move {
            let empty_args = to_value(&{}).unwrap();
            let response = invoke("maximize", empty_args).await;
            if response.is_err() {
                web_sys::console::error_1(&response.unwrap_err());
            }
        });
    });

    let on_close = Callback::from(move |_event: MouseEvent| {
        spawn_local(async move {
            let empty_args = to_value(&{}).unwrap();
            let response = invoke("close", empty_args).await;
            if response.is_err() {
                web_sys::console::error_1(&response.unwrap_err());
            }
        });
    });
    
    html! {
        <div data-tauri-drag-region="true" class="w-full fixed flex justify-between p-1 bg-sky-950">
            <div>
                <span class="text-sky-50 ml-3 font-bold text-md">{"Chat Doc"}</span>
            </div>
            <div class="flex justify-normal space-x-2 mr-3">
                <div onclick={ on_minimize }>
                    <MinimizeImage class="size-5 rounded-md fill-sky-50 hover:fill-sky-950 hover:bg-sky-50" />
                </div>
                <div onclick={ on_maximize }>
                    <MaximizeImage class="size-5 rounded-md fill-sky-50 hover:fill-sky-950 hover:bg-sky-50" />
                </div>
                <div onclick={ on_close }>
                    <CloseImage class="size-5 rounded-md fill-sky-50 hover:fill-sky-950 hover:bg-sky-50" />
                </div>
            </div>
        </div>
    }
}