mod content;

use content::content;
use leptos::{html::*, *};

use crate::bases::BaseConversion;

#[derive(Clone, Debug)]
enum OpenState {
    Open,
    Closed,
}

#[component]
pub fn OutputDetails(base_conversion: Memo<BaseConversion>) -> impl IntoView {
    let (is_open, set_is_open) = create_signal(OpenState::Closed);
    let close = move || set_is_open(OpenState::Closed);
    let open = move |_| set_is_open(OpenState::Open);

    move || match (base_conversion().output_string(), is_open()) {
        (Ok(o), OpenState::Open) => div().child(content(
            create_memo(move |_| o.clone()),
            create_memo(move |_| base_conversion().output_base).into(),
            close,
        )),
        (Ok(_), OpenState::Closed) => {
            div().child(button().on(ev::click, open).child("Show Output Details"))
        }
        (Err(_), _) => div(),
    }
}
