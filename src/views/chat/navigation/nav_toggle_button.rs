use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(NavToggleButton)]
pub fn nav_toggle_button (props: &Props) -> Html {
    html! {
        <a href="#" class="min-w-16 min-h-14 pr-2 flex items-center justify-center rounded-l-full hover:bg-sky-50">
            { props.children.clone() }
        </a>
    }
}