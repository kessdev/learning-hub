use wasm_bindgen::prelude::*;
use wasm_bindgen_futures::spawn_local;
use serde_wasm_bindgen::to_value;
use serde::{Deserialize, Serialize};

use crate::{models::topic::topic_domain::{Topic, TopicRepository}, utils::callback::ResponseCallback};

#[wasm_bindgen]
extern "C" {
    #[wasm_bindgen(js_namespace = ["window", "__TAURI__", "tauri"], catch)]
    async fn invoke(cmd: &str, args: JsValue) -> Result<JsValue, JsValue>;
}

#[derive(Serialize, Deserialize)]
struct Message {
    topic: Topic,
}

pub struct TopicRepositoryImpl {}

impl TopicRepository for TopicRepositoryImpl {
    fn save(&self, topic: Topic, callback: ResponseCallback<(), String>) {
        spawn_local(async move {
            let topic_cloned = topic.clone();
            let message = Message {
                topic: topic_cloned,
            };
            let args = to_value(&message).unwrap();
            let result = invoke("save_topic", args).await;
            match result {
                Ok(_) => {
                    callback.ok.emit(());
                },
                Err(_) => {
                    callback.error.emit("Error saving topic".to_string());
                }
            };
        });
    }
}

impl TopicRepositoryImpl {
    pub fn new() -> Self {
        Self {}
    }
}