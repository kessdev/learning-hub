use std::ops::Deref;
use serde::{Deserialize, Serialize};
use bounce::{use_atom, use_atom_value, Atom};
use web_sys::SubmitEvent;
use yew::{function_component, html, use_state, AttrValue, Callback, Html};

use crate::{components::{
    button::FormButton, image::LibraryImage, modal::Modal, text::{TextArea, TextField}
}, models::{injector::Injector, topic::topic_domain::Topic}, utils::callback::ResponseCallback};

#[derive(Atom, PartialEq, Clone, Debug, Serialize, Deserialize)]
pub struct TopicFormStatus {
    pub form_title: String,
    pub hidden: bool,
    pub id: Option<String>,
    pub title: String,
    pub description: String,
}

impl Default for TopicFormStatus {
    fn default() -> Self {
        Self {
            form_title: String::new(),
            hidden: true,
            id: None,
            title: String::new(),
            description: String::new(),
        }
    }
}

#[function_component(TopicForm)]
pub fn topic_form() -> Html {
    let injector = Injector::new();

    let topic_form_data = use_atom_value::<TopicFormStatus>();
    let topic = TopicFormStatus {
        form_title: topic_form_data.form_title.clone(),
        hidden: topic_form_data.hidden.clone(),
        id: topic_form_data.id.clone(),
        title: topic_form_data.title.clone(),
        description: topic_form_data.description.clone(),
    };

    let topic_state = use_state(|| topic);

    let topic_state_cloned = topic_state.clone();
    let title_changed = Callback::from(move |title: String| {
        let mut topic = topic_state_cloned.deref().clone();
        topic.title = title;
        topic_state_cloned.set(topic);
    });

    let topic_state_cloned = topic_state.clone();
    let description_changed = Callback::from(move |description: String| {
        let mut topic = topic_state_cloned.deref().clone();
        topic.description = description;
        topic_state_cloned.set(topic);
    });

    let topic_form_updater = use_atom::<TopicFormStatus>();
    let topic_service = injector.topic_service;
    let topic_state_cloned = topic_state.clone();
    let handle_submit = Callback::from(move |event: SubmitEvent| {
        event.prevent_default();
        let topic_form_updater = topic_form_updater.clone();
        let topic_form_status = topic_state_cloned.deref().clone();
        let topic = Topic::new(topic_form_status.id.clone(), 
            topic_form_status.title.clone(), topic_form_status.description.clone());

        let response_callback = ResponseCallback {
            ok: Callback::from(move |_| {
                let mut topic_form_status = topic_form_status.clone();
                topic_form_status.hidden = true;
                topic_form_updater.set(topic_form_status);
            }),
            error: Callback::from(|_error: String| {}),
        };

        topic_service.save(topic, response_callback);
    });

    let topic_state_cloned = topic_state.clone();
    let topic = topic_state_cloned.deref().clone();
        
    html! {
        <>
            <Modal hidden={ topic_form_data.hidden.clone() }>
                <form onsubmit={ handle_submit }>
                    <div class="flex flex-row p-5">
                        <div class="flex-none">
                            <LibraryImage class="size-12 rounded-full fill-sky-950" />
                        </div>
                        <div class="grow pl-5">
                            <form>
                                <input type="hidden" name="id" value={ topic.id } />
                                    <TextField label="Topic" 
                                        name="topic" 
                                        value={ topic.title }
                                        placeholder="Topic name" 
                                        on_change={ title_changed } 
                                        error_message="campo vacio" />
                                    <TextArea label="Description" 
                                        name="description" 
                                        value={ topic.description } 
                                        placeholder="Topic description" 
                                        on_change={ description_changed }
                                        error_message={ None::<AttrValue> } />
                            </form>
                        </div>
                    </div>
                    <div class="flex justify-end space-x-3 p-3 bg-sky-100 w-full">
                        <FormButton button_type="submit"
                            value="Save"
                            class="max-w-20" />
                        <FormButton button_type="button"
                            value="Cancel"
                            class="max-w-20" />
                    </div>
                </form>
            </Modal>
        </>
    }
}
