use bigdecimal::BigDecimal;
use leptos::{html::*, *};

use crate::bases::{pow, rep_to_digit_exponent_pairs};

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
                "Representing base-{} digits as base-10 numbers:",
                base()
            )))
            .child(move || {
                digit_exponent_pairs()
                    .into_iter()
                    .map(|(c, i)| {
                        td().child(span().classes("red").child(digit_to_value(c)))
                            .child('(')
                            .child(move || base().to_string())
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
                .child("Close Details"),
        )
        .child(p().child("The output value (changes in red) can be converted to base-10:"))
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
                                base()
                            )))
                            .child(move || {
                                digit_exponent_pairs()
                                    .into_iter()
                                    .map(|(c, i)| {
                                        td().child(span().classes("red").child(match c.len() {
                                            1 => c.to_string(),
                                            _ => format!("[{c}]"),
                                        }))
                                        .child('(')
                                        .child(move || base().to_string())
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
                                    .child("Evaluating the exponents on the base:"),
                            )
                            .child(move || {
                                digit_exponent_pairs()
                                    .into_iter()
                                    .map(|(c, i)| {
                                        td().child(span().child(digit_to_value(c)))
                                            .child('(')
                                            .child(
                                                span()
                                                    .classes("red")
                                                    .child(move || pow(&base(), i).to_string()),
                                            )
                                            .child(')')
                                    })
                                    .intersperse_with(|| td().child("+"))
                                    .collect_view()
                            })
                            .child(filler()),
                        )
                        .child(
                            tr().child(td().classes("align-end").child("Multiplying:"))
                                .child(move || {
                                    digit_exponent_pairs()
                                        .into_iter()
                                        .map(|(c, i)| {
                                            td().child(span().classes("red").child(
                                                (pow(&base(), i) * digit_to_value(c)).to_string(),
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
                                            .classes("red")
                                            .child(move || {
                                                digit_exponent_pairs()
                                                    .into_iter()
                                                    .map(|(c, i)| {
                                                        pow(&base(), i) * digit_to_value(c)
                                                    })
                                                    .sum::<BigDecimal>()
                                                    .to_string()
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
