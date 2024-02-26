use std::ops::Deref;

use yew::{function_component, html, use_state, Callback, Html};

use crate::components::{image::SearchImage, text::TextFieldWithImage};

#[function_component(Search)]
pub fn search() -> Html {
    let search_state = use_state(|| String::new());

    let search_state_clone = search_state.clone();
    let search_changed = Callback::from(move |text: String| {
        search_state_clone.set(text);
    });

    let search_state_clone = search_state.clone();

    html! {
        <TextFieldWithImage 
            name="search" 
            value={ search_state_clone.deref().clone() } 
            placeholder="Search.."
            on_change={ search_changed }>
            <SearchImage class="w-5 h-5 stroke-sky-300 fill-none" />
        </TextFieldWithImage>
    }
}
