use yew::{function_component, html, Html};

use crate::views::chat::{navigation::nav_panel::NavPanel, sidebar::root::Sidebar};

#[function_component(ChatView)]
pub fn chat_view() -> Html {
    html! {
        <div class="grow flex flex-row">
            <NavPanel />
            <Sidebar />
        </div>
    }
}