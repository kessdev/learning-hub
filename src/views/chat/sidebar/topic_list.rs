use web_sys::MouseEvent;
use yew::{function_component, html, Callback, Html, Properties};

use crate::{models::topic::topic_domain::Topic, views::chat::sidebar::topic_item::TopicItem};

#[derive(Properties, PartialEq, Clone, Debug)]
pub struct Props {
    pub action: Callback<MouseEvent>,
    pub topics: Vec<Topic>,
}

#[function_component(TopicList)]
pub fn topic_list(props: &Props) -> Html {
    html! {
        <div class="w-full h-full scroll-smooth scrollbar-hidden" style="overflow-x: visible; overflow-y: scroll">
            
        </div>
    }
}

/*

{
                props.topics.into_iter().map(|topic| {
                    html!{
                        <TopicItem id={ topic.id.clone() } 
                            title={ topic.title.clone() } 
                            description={ topic.description.clone() } 
                            action={ props.action.clone() }/>
                    }
                }).collect::<Html>()
            }
*/