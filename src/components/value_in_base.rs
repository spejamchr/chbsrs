use bigdecimal::BigDecimal;
use leptos::{html::*, *};

use crate::bases::rounded_string;

pub fn value_in_base(val: Memo<Result<String, String>>, base: Signal<BigDecimal>) -> impl IntoView {
    move || {
        div().classes("value").child(match val() {
            Ok(v) => span().child(
                code()
                    .attr("title", move || {
                        format!("Value in Base-{}", rounded_string(base(), None))
                    })
                    .child(v)
                    .child(span().inner_html("&nbsp"))
                    .child(sub().child(move || rounded_string(base(), None))),
            ),
            Err(e) => span().child(e),
        })
    }
}
