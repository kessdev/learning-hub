use yew::{classes, function_component, html, Html, Properties};

#[derive(Properties, PartialEq)]
pub struct Props {
    pub children: Html,
    pub hidden: bool,
}

#[function_component(Modal)]
pub fn modal(props: &Props) -> Html {
    html! {
        <div class={classes!("relative","z-10", props.hidden.then(|| Some("hidden")),)} 
            aria-labelledby="modal-title" role="dialog" aria-modal="true">
            <div class="fixed inset-0 bg-sky-50 bg-opacity-75 transition-opacity" />
            <div class="fixed inset-0 z-10 w-screen overflow-y-auto">
                <div class="flex min-h-full items-end justify-center p-3 text-center items-center">
                    <div class="relative overflow-hidden rounded-md text-left shadow-md shadow-sky-950 w-full max-w-lg ring-1 ring-sky-200 bg-sky-50">
                        { props.children.clone() }
                    </div>
                </div>
            </div>
        </div>
    }
}