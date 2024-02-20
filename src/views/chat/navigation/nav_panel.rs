use yew::{function_component, html, Html};

use crate::components::image::{ChatImage, LibraryImage, MoonImage, SettingsImage, UserImage};
use crate::views::chat::navigation::{nav_toggle_button::NavToggleButton, nav_button::NavButton};

#[function_component(NavPanel)]
pub fn nav_panel() -> Html {
    html! {
        <div class="h-full items-center flex flex-col bg-sky-100">
            <nav class="grow space-y-2 ml-2 mt-4">
                <NavToggleButton>
                    <LibraryImage class="w-6 h-6 fill-sky-300" />
                </NavToggleButton>
                <NavToggleButton>
                    <ChatImage class="w-6 h-6 fill-sky-300" />
                </NavToggleButton>
            </nav>

            <nav class="flex-none space-y-2 mx-2 mb-4">
                <NavButton>
                    <MoonImage class="w-6 h-6 fill-sky-300" />
                </NavButton>
                <NavButton>
                    <SettingsImage class="w-6 h-6 fill-sky-300" />
                </NavButton>
                <NavButton>
                    <UserImage class="w-6 h-6 fill-sky-300" />
                </NavButton>
            </nav>
        </div>
    }
}