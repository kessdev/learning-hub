use web_sys::{Event, MouseEvent};
use yew::{function_component, html, use_state, AttrValue, Callback, Html, Properties};

use crate::components::{image::{EllipsisVerticalImage, PencilImage, TrashImage}, button::ImageButton, popover::Popover};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub action: Callback<MouseEvent>,
    pub id: Option<AttrValue>,
    pub title: AttrValue,
    pub description: AttrValue,
}

#[function_component(TopicItem)]
pub fn topic_item(props: &Props) -> Html {
    let hidden_options_state = use_state(|| true);
    let hidden_options_state_clone = hidden_options_state.clone();
    let handle_show_options = Callback::from(move |_event: MouseEvent| {
        hidden_options_state_clone.set(false);
    });
    let hidden_options_state_clone = hidden_options_state.clone();
    let handle_hide_options = Callback::from(move |_event: Event| {
        hidden_options_state_clone.set(true);
    });
    
    html! {
        <div class="flex flex-row w-full my-3">
            <div class="flex flex-row w-full ml-3 mr-4 pl-5 pr-3 py-5 pr-0 rounded-md text-sky-950 hover:bg-sky-100 hover:text-sky-950">
                <a href="#" class="flex flex-col grow">
                    <article class="w-full text-wrap">
                        <h3 class="font-semibold">
                                {props.title.clone()}
                        </h3>
                        <p>
                            {props.description.clone()}
                        </p>
                    </article>
                </a>
                <div class="relative inline-block">
                    <ImageButton class="size-5 rounded-full hover:bg-sky-50" 
                        on_click={ handle_show_options }>
                        <EllipsisVerticalImage class="size-5 stroke-sky-300 stroke-2" />
                    </ImageButton>
                    <Popover hidden={ *hidden_options_state } 
                        class="top-5 right-0 w-40 shadow-sky-950 ring-sky-200 bg-sky-50"
                        on_hide={ handle_hide_options }>
                        <ul>
                            <li>
                                <a href="#"
                                    class="flex flex-row text-sky-950 block px-2 py-2 rounded-md text-sm hover:bg-sky-100"
                                    id="edit">
                                    <PencilImage class="size-5 mx-3 fill-sky-950" />
                                    {"Edit"}
                                </a>
                            </li>
                            <li>
                                <a href="#" 
                                    class="flex flex-row text-sky-950 block px-2 py-2 rounded-md text-sm hover:bg-sky-100" 
                                    id="delete">
                                    <TrashImage class="size-5 mx-3 fill-sky-950" />
                                    {"Delete"}
                                </a>
                            </li>
                        </ul>
                    </Popover>
                </div>
            </div>
        </div>
    }
}
