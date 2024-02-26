use wasm_bindgen::JsCast;
use web_sys::{Event, HtmlInputElement, HtmlTextAreaElement};
use yew::{classes, function_component, html, AttrValue, Callback, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct TextFieldProps {
    pub on_change: Callback<String>,
    pub label: AttrValue,
    pub name: AttrValue,
    pub value: AttrValue,
    pub placeholder: AttrValue,
    #[prop_or(None)]
    pub error_message: Option<AttrValue>,
}

#[function_component(TextField)]
pub fn text_field(props: &TextFieldProps) -> Html {
    let on_change = props.on_change.clone();
    let handle_on_change = Callback::from(move |event: Event| {
        let value = event.target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        on_change.emit(value);
    });

    let has_error = props.error_message.is_some();
    
    html! {
        <>
            <label for={ props.name.clone() } 
                class={ 
                    classes!(String::from("text-sm font-semibold mt-4"),
                    if has_error { "text-red-700" } else { "text-sky-950" }
                )}>{ props.label.clone() }</label>

            <input type="text"
                name={ props.name.clone() }
                value={ props.value.clone() }
                placeholder={ props.placeholder.clone() }
                onchange={ handle_on_change }
                class={ 
                    classes!(String::from("w-full h-11 py-3 px-4 rounded-lg shadow-none placeholder:italic focus:outline focus:outline-1"),
                    if has_error { "bg-red-100 placeholder:text-red-300 text-red-700 focus:outline-red-300" } 
                    else { "bg-sky-100 placeholder:text-sky-300 text-sky-950  focus:outline-sky-300" }
                )} />
            {
                if has_error {
                    html! {
                        <p class="text-sm mt-1 mb-2 px-4 text-red-700">{ props.error_message.clone().unwrap() }</p>
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}

#[derive(Properties, PartialEq)]
pub struct TextFieldWithImageProps {
    pub children: Html,
    pub on_change: Callback<String>,
    pub name: AttrValue,
    pub value: AttrValue,
    pub placeholder: AttrValue,
}

#[function_component(TextFieldWithImage)]
pub fn text_field_with_image(props: &TextFieldWithImageProps) -> Html {
    let on_change = props.on_change.clone();
    let handle_on_change = Callback::from(move |event: Event| {
        let value = event.target()
            .unwrap()
            .unchecked_into::<HtmlInputElement>()
            .value();
        on_change.emit(value);
    });

    html! {
        <div class="relative px-5 pb-5">
            <i class="absolute ml-3 mt-3 text-center">
                { props.children.clone() }
            </i>
            <input type="text"
                name={ props.name.clone() }
                value={ props.value.clone() }
                placeholder={ props.placeholder.clone() }
                onchange={ handle_on_change }
                class="w-full h-11 py-3 pl-11 pr-4 rounded-lg shadow-none bg-sky-100 placeholder:italic placeholder:text-sky-300 text-sky-950 focus:outline-sky-300 focus:outline focus:outline-1" />
        </div>
    }
}

#[derive(Properties, PartialEq)]
pub struct TextAreaProps {
    pub on_change: Callback<String>,
    pub label: String,
    pub name: String,
    pub value: String,
    pub placeholder: String,
    #[prop_or(None)]
    pub error_message: Option<AttrValue>,
}

#[function_component(TextArea)]
pub fn text_area(props: &TextAreaProps) -> Html {
    let on_change = props.on_change.clone();
    let handle_on_change = Callback::from(move |event: Event| {
        let value = event.target()
            .unwrap()
            .unchecked_into::<HtmlTextAreaElement>()
            .value();
        on_change.emit(value);
    });

    let has_error = props.error_message.is_some();

    html! {
        <>
            <label for={ props.name.clone() } 
                class={ classes!(String::from("text-sm font-semibold mt-4"),
                    if has_error { "text-red-700" } else { "text-sky-950" }
                )}>{ props.label.clone() }</label>

            <textarea name={ props.name.clone() }
                placeholder={ props.placeholder.clone() }
                onchange={ handle_on_change }
                class={ classes!(String::from("w-full py-3 px-4 m-0 rounded-lg shadow-none placeholder:italic focus:outline focus:outline-1"),
                    if has_error { "bg-red-100 placeholder:text-red-300 text-red-700 focus:outline-red-300" } 
                    else { "bg-sky-100 placeholder:text-sky-300 text-sky-950  focus:outline-sky-300" }
                )}>{ props.value.clone() }</textarea>

            {
                if has_error {
                    html! {
                        <p class="text-sm mt-1 mb-2 px-4 text-red-700">{ props.error_message.clone().unwrap() }</p>
                    }
                } else {
                    html! {}
                }
            }
        </>
    }
}