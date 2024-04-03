use bigdecimal::BigDecimal;
use leptos::{html::*, *};

use super::rounded_bignum::rounded_bignum;

pub fn value_in_base(val: Memo<Result<String, String>>, base: Memo<BigDecimal>) -> impl IntoView {
    move || {
        div().classes("value").child(match val() {
            Ok(v) => span().child(
                code()
                    .child(span().attr("title", &v).child(v))
                    .child(span().inner_html("&nbsp"))
                    .child(sub().child(move || rounded_bignum(base(), None))),
            ),
            Err(e) => span().child(e),
        })
    }
}
