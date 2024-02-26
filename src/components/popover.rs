use web_sys::Event;
use yew::{classes, function_component, html, use_node_ref, AttrValue, Callback, Html, Properties};
use yew_hooks::use_click_away;

#[derive(Properties, Clone, PartialEq)]
pub struct Props {
    pub children: Html,
    pub hidden: bool,
    pub class: AttrValue,
    pub on_hide: Callback<Event>,
}

#[function_component(Popover)]
pub fn popover(props: &Props) -> Html {
    let node = use_node_ref();
    let on_hide = props.on_hide.clone();
    use_click_away(node.clone(), move |event: Event| {
        on_hide.emit(event);
    });

    html! {
        <div class={ classes!(
            String::from("absolute tz-10 rounded-md shadow-md ring-1"), props.class.clone(), props.hidden.then(|| Some("hidden"))) }
            ref={ node }>
            { props.children.clone() }
        </div>
    }
}