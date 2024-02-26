use yew::{function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
}

#[function_component(NavButton)]
pub fn nav_button (props: &Props) -> Html {
    html! {
        <a href="#" 
            class="min-w-14 min-h-14 flex items-center justify-center rounded-full hover:bg-sky-50">
            { props.children.clone() }
        </a>
    }
}