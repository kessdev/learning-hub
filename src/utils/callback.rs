use yew::Callback;

pub struct ResponseCallback<O, E> {
    pub ok: Callback<O>,
    pub error: Callback<E>,
}