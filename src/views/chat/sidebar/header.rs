use bounce::use_atom;
use web_sys::MouseEvent;
use yew::{function_component, html, AttrValue, Callback, Html, Properties};

use crate::{
    components::{image::PlusImage, button::ImageButton}, 
    views::chat::sidebar::topic_form::TopicFormStatus,
    //views::chat::sidebar::topic_form::TopicFormAction
};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub title: AttrValue,
    pub alt: AttrValue,
}

#[function_component(Header)]
pub fn header (props: &Props) -> Html {
    let topic_form_updater = use_atom::<TopicFormStatus>();
    let handle_on_click = Callback::from(move |_event: MouseEvent| {
        topic_form_updater.set(TopicFormStatus {
            form_title: String::from("New Topic"),
            hidden: false,
            id: None,
            title: String::new(),
            description: String::new(),
            //action: TopicFormAction::New,
        });
    });
    
    html! {
        <div class="flex justify-between p-5">
            <p class="font-sans text-lg text-sky-950">
                { props.title.clone() }
            </p>
            <ImageButton class="size-5 rounded-full hover:bg-sky-100" 
                on_click={ handle_on_click }>
                <PlusImage class="size-5 stroke-sky-300 stroke-2" />
            </ImageButton>
            { props.children.clone() }
        </div>
    }
}