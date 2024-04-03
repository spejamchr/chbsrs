use bigdecimal::BigDecimal;
use leptos::{html::*, *};
use web_sys::Event;

use crate::bases::BaseConversion;

use super::rounded_bignum::rounded_bignum;

#[component]
pub fn HomeInputs(
    base_conversion: Memo<BaseConversion>,
    #[prop(into)] set_input_string: WriteSignal<String>,
    #[prop(into)] set_input_base_string: WriteSignal<String>,
    #[prop(into)] set_output_base_string: WriteSignal<String>,
    #[prop(into)] accurate_conversion: ReadSignal<Option<String>>,
    #[prop(into)] set_accurate_conversion: WriteSignal<Option<String>>,
) -> impl IntoView {
    move || {
        table()
            .classes("inputs")
            .child(
                thead().child(
                    tr().child(th().child("Value in Base-10:"))
                        .child(th().child(div().classes("value").child(move || {
                            match base_conversion().base_10_value() {
                                Ok(v) => span().child(
                                    code()
                                        .child(rounded_bignum(v, None))
                                        .child(span().inner_html("&nbsp"))
                                        .child(sub().child(move || {
                                            rounded_bignum(BigDecimal::from(10), None)
                                        })),
                                ),
                                Err(e) => span().child(e),
                            }
                        }))),
                ),
            )
            .child(
                tfoot()
                    .on(ev::mouseover, move |_| {
                        set_accurate_conversion(base_conversion().output_string_accurate().ok())
                    })
                    .child(
                        tr().child(th().child("Output Value:"))
                            .child(th().child(move || {
                                div().classes("value").child(
                                    match base_conversion().output_string() {
                                        Ok(v) => span().child(
                                            code()
                                                .child(span().child(&v).attr("tabindex", "0").attr(
                                                    "title",
                                                    move || {
                                                        accurate_conversion()
                                                            .unwrap_or_else(|| v.clone())
                                                    },
                                                ))
                                                .child(span().inner_html("&nbsp"))
                                                .child(sub().child(move || {
                                                    rounded_bignum(
                                                        base_conversion().output_base,
                                                        None,
                                                    )
                                                })),
                                        ),
                                        Err(e) => span().child(e),
                                    },
                                )
                            })),
                    ),
            )
            .child(
                tbody()
                    .child(
                        tr().child(
                            td().child(label().attr("for", "InputValue").child("Input Value")),
                        )
                        .child(
                            td().child(
                                input()
                                    .id("InputValue")
                                    .style("width", move || {
                                        format!(
                                            "{}ch",
                                            2 + base_conversion()
                                                .input_string
                                                .chars()
                                                .count()
                                                .max(18)
                                        )
                                    })
                                    .attr("type", "text")
                                    .attr("value", move || base_conversion().input_string)
                                    .on(ev::input, move |ev| {
                                        set_input_string(event_target_value(&ev))
                                    }),
                            ),
                        ),
                    )
                    .child(
                        tr().child(
                            td().child(label().attr("for", "InputBase").child("Input Base")),
                        )
                        .child(
                            td().child(
                                input()
                                    .id("InputBase")
                                    .attr("type", "text")
                                    .attr("value", move || base_conversion().input_base_string)
                                    .on(ev::input, update_base(set_input_base_string)),
                            ),
                        ),
                    )
                    .child(
                        tr().child(
                            td().child(label().attr("for", "OutputBase").child("Output Base")),
                        )
                        .child(
                            td().child(
                                input()
                                    .id("OutputBase")
                                    .attr("type", "text")
                                    .attr("value", move || base_conversion().output_base_string)
                                    .on(ev::input, update_base(set_output_base_string)),
                            ),
                        ),
                    ),
            )
    }
}

fn update_base<SF>(setter: SF) -> impl Fn(Event)
where
    SF: Fn(String),
{
    move |ev| {
        setter(event_target_value(&ev));
    }
}
