use leptos::{html::*, *};

pub fn value_in_base(val: Memo<Result<String, String>>, base: Signal<f64>) -> impl IntoView {
    move || {
        div().classes("value").child(match val() {
            Ok(v) => span().child(
                code()
                    .attr("title", format!("Base-{}", base()))
                    .child(v)
                    .child(span().inner_html("&nbsp"))
                    .child(sub().child(base)),
            ),
            Err(e) => span().child(e),
        })
    }
}
