use yew::{function_component, html, Html};

use crate::views::chat::navigation::nav_panel::NavPanel;

#[function_component(ChatView)]
pub fn chat_view() -> Html {
    html! {
        <div class="h-full flex flex-row overflow-hidden">
            <NavPanel />
        </div>
    }
}