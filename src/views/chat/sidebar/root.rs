use yew::{function_component, html, Html};

use crate::views::chat::sidebar::{header::Header, topic_form::TopicForm};

#[function_component(Sidebar)]
pub fn sidebar () -> Html {
    html! {
        <div class="flex flex-col w-80 h-full">
            <Header title="Topics" alt="Topics">
                <TopicForm />
            </Header>
            
        </div>
    }
}

/*
<Header title="Topics" alt="Topics" show_topic_form={ show_topic_form }>
                <TopicForm hidden={ *hide_form_state } { submit } />
            </Header>
<Search />
            <TopicList topics={ topics_state_cloned } action={ topic_list_action }/>
*/