use web_sys::MouseEvent;
use yew::{classes, function_component, html, AttrValue, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct ImageButtonProps {
    pub children: Html,
    pub on_click: Callback<MouseEvent>,
    pub class: AttrValue,
}

#[function_component(ImageButton)]
pub fn image_button(props: &ImageButtonProps) -> Html {
    let create_data = props.on_click.clone();
    let handle_on_click = Callback::from(move |event: MouseEvent| create_data.emit(event));

    html! {
        <a onclick={ handle_on_click } class={ props.class.clone() }>
            { props.children.clone() }
        </a>
    }
}

#[derive(Properties, PartialEq)]
pub struct FormButtonProps {
    pub button_type: AttrValue,
    pub value: AttrValue,
    pub class: AttrValue,
}

#[function_component(FormButton)]
pub fn form_button(props: &FormButtonProps) -> Html {
    html! {
        <input type={ props.button_type.clone() }
            value={ props.value.clone() }
            class={ classes!(
                String::from("inline-flex w-full justify-center rounded-lg bg-sky-200 px-3 py-3 text-sm font-semibold text-sky-950 ring-0 hover:bg-sky-50 w-auto"),
                props.class.clone())
             } />
    }
}