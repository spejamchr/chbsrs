use std::num::NonZeroU64;

use bigdecimal::BigDecimal;
use leptos::{html::*, *};

use crate::bases::{pow, rep_to_digit_exponent_pairs, rounded_string};

pub fn content<G>(output: Memo<String>, base: Signal<BigDecimal>, close: G) -> impl IntoView
where
    G: Fn() + 'static,
{
    let all_pairs = rep_to_digit_exponent_pairs(&output());

    let digit_exponent_pairs = create_memo(move |_| -> Vec<_> {
        rep_to_digit_exponent_pairs(&output())
            .into_iter()
            .take(5)
            .collect()
    });

    let needs_filler = all_pairs.len() > digit_exponent_pairs().len();
    let filler = move || match needs_filler {
        true => Some(
            td().child(span().classes("right-space").child("+"))
                .child(span().child("…")), // ellide
        ),
        false => None,
    };

    let digit_to_value = |s: String| s.parse().or_else(|_| u32::from_str_radix(&s, 36)).unwrap();

    let digit_conversion = match base() > BigDecimal::from(10) {
        true => Some(
            tr().child(td().classes("align-end").child(format!(
                "Represent base-{} digits as base-10 numbers:",
                rounded_string(base(), None)
            )))
            .child(move || {
                digit_exponent_pairs()
                    .into_iter()
                    .map(|(c, i)| {
                        td().child(span().classes("highlight").child(digit_to_value(c)))
                            .child('(')
                            .child(move || rounded_string(base(), None))
                            .child(sup().child(i))
                            .child(')')
                    })
                    .intersperse_with(|| td().child("+"))
                    .collect_view()
            })
            .child(filler()),
        ),
        false => None,
    };

    div()
        .child(
            button()
                .on(ev::click, move |_| close())
                .child("Hide Output Details"),
        )
        .child(p().child("The output value can be converted to base-10:"))
        .child(
            table()
                .child(
                    thead().child(
                        tr().child(th().child("Step")).child(
                            th().attr("colspan", move || digit_exponent_pairs().len() * 2)
                                .child("Digits"),
                        ),
                    ),
                )
                .child(
                    tbody()
                        .child(
                            tr().child(td().classes("align-end").child(format!(
                                "Output value w/base-{} positioned values:",
                                rounded_string(base(), None)
                            )))
                            .child(move || {
                                digit_exponent_pairs()
                                    .into_iter()
                                    .map(|(c, i)| {
                                        td().child(span().classes("highlight").child(
                                            match c.len() {
                                                1 => c.to_string(),
                                                _ => format!("[{c}]"),
                                            },
                                        ))
                                        .child('(')
                                        .child(move || rounded_string(base(), None))
                                        .child(sup().child(i))
                                        .child(')')
                                    })
                                    .intersperse_with(|| td().child("+"))
                                    .collect_view()
                            })
                            .child(filler()),
                        )
                        .child(digit_conversion)
                        .child(
                            tr().child(
                                td().classes("align-end")
                                    .child("Evaluate the exponents on the base:"),
                            )
                            .child(move || {
                                digit_exponent_pairs()
                                    .into_iter()
                                    .map(|(c, i)| {
                                        td().child(span().child(digit_to_value(c)))
                                            .child('(')
                                            .child(span().classes("highlight").child(move || {
                                                rounded_string(pow(&base(), i), NonZeroU64::new(8))
                                            }))
                                            .child(')')
                                    })
                                    .intersperse_with(|| td().child("+"))
                                    .collect_view()
                            })
                            .child(filler()),
                        )
                        .child(
                            tr().child(td().classes("align-end").child("Multiply to get:"))
                                .child(move || {
                                    digit_exponent_pairs()
                                        .into_iter()
                                        .map(|(c, i)| {
                                            td().child(span().classes("highlight").child(
                                                rounded_string(
                                                    pow(&base(), i) * digit_to_value(c),
                                                    NonZeroU64::new(8),
                                                ),
                                            ))
                                        })
                                        .intersperse_with(|| td().child("+"))
                                        .collect_view()
                                })
                                .child(filler()),
                        ),
                )
                .child(
                    tfoot().child(
                        tr().child(th().classes("align-end").child("Adding everything:"))
                            .child(
                                th().attr("align", "left")
                                    .attr("colspan", move || digit_exponent_pairs().len() * 2)
                                    .child(
                                        span()
                                            .classes("highlight")
                                            .child(move || {
                                                rounded_string(
                                                    digit_exponent_pairs()
                                                        .into_iter()
                                                        .map(|(c, i)| {
                                                            pow(&base(), i) * digit_to_value(c)
                                                        })
                                                        .sum::<BigDecimal>(),
                                                    None,
                                                )
                                            })
                                            .child(match needs_filler {
                                                true => Some(" + …"), // ellide
                                                false => None,
                                            }),
                                    ),
                            ),
                    ),
                ),
        )
}
