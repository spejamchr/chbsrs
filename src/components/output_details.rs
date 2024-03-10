mod content;

use bigdecimal::BigDecimal;
use content::content;
use leptos::{html::*, *};

#[derive(Clone, Debug)]
enum OpenState {
    Open,
    Closed,
}

#[component]
pub fn OutputDetails(
    output: Memo<Result<String, String>>,
    #[prop(into)] base: Signal<BigDecimal>,
) -> impl IntoView {
    let (is_open, set_is_open) = create_signal(OpenState::Closed);
    let close = move || set_is_open(OpenState::Closed);
    let open = move |_| set_is_open(OpenState::Open);

    move || match (output(), is_open()) {
        (Ok(o), OpenState::Open) => {
            div().child(content(create_memo(move |_| o.clone()), base, close))
        }
        (Ok(_), OpenState::Closed) => {
            div().child(button().on(ev::click, open).child("Open Details"))
        }
        (Err(_), _) => div(),
    }
}
