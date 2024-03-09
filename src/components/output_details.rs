mod content;

use content::content;
use leptos::{html::*, *};

#[derive(Clone, Debug)]
enum OpenState {
    Open,
    Closed,
}

#[component]
pub fn OutputDetails<F>(output: F, #[prop(into)] base: Signal<f64>) -> impl IntoView
where
    F: 'static + Fn() -> Result<String, String>,
{
    let (is_open, set_is_open) = create_signal(OpenState::Closed);
    let close = move || set_is_open(OpenState::Closed);
    let open = move |_| set_is_open(OpenState::Open);

    move || match (output(), is_open()) {
        (Ok(o), OpenState::Open) => div().child(content(move || o.clone(), base, close)),
        (Ok(_), OpenState::Closed) => {
            div().child(button().on(ev::click, open).child("Open Details"))
        }
        (Err(_), _) => div(),
    }
}
