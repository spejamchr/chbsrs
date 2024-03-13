use bigdecimal::BigDecimal;
use leptos::{html::*, *};

pub fn rounded_string(num: BigDecimal) -> String {
    let limit = 8;
    let (_, scale) = num.as_bigint_and_exponent();
    if scale > limit {
        format!("{}…", num.with_scale(limit)) // ellide
    } else {
        num.to_string()
    }
}

pub fn value_in_base(val: Memo<Result<String, String>>, base: Signal<BigDecimal>) -> impl IntoView {
    move || {
        div().classes("value").child(match val() {
            Ok(v) => span().child(
                code()
                    .attr("title", move || format!("Base-{}", rounded_string(base())))
                    .child(v)
                    .child(span().inner_html("&nbsp"))
                    .child(sub().child(move || rounded_string(base()))),
            ),
            Err(e) => span().child(e),
        })
    }
}
