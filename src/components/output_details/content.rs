use leptos::{html::*, *};

use crate::bases::rep_to_digit_exponent_pairs;

pub fn content<F, G>(output: F, base: Signal<f64>, close: G) -> impl IntoView
where
    F: 'static + Fn() -> String,
    G: 'static + Fn() -> (),
{
    let all_pairs = rep_to_digit_exponent_pairs(&output());

    let digit_exponent_pairs: Vec<_> = rep_to_digit_exponent_pairs(&output())
        .into_iter()
        .take(5)
        .collect();

    let needs_filler = all_pairs.len() > digit_exponent_pairs.len();
    let filler = move || match needs_filler {
        true => Some(
            td().child(span().classes("right-space").child("+"))
                .child(span().child("...")),
        ),
        false => None,
    };

    let digit_to_value = |s| {
        usize::from_str_radix(s, 10)
            .or_else(|_| usize::from_str_radix(s, 36))
            .unwrap()
    };

    let digit_conversion = match base() > 10.0 {
        true => Some(
            tr().child(td().classes("align-end").child(format!(
                "Representing base-{} digits as base-10 numbers:",
                base()
            )))
            .child(
                digit_exponent_pairs
                    .iter()
                    .map(|(c, i)| {
                        td().child(span().classes("red").child(digit_to_value(c)))
                            .child('(')
                            .child(base())
                            .child(sup().child(*i))
                            .child(')')
                    })
                    .intersperse_with(|| td().child("+"))
                    .collect_view(),
            )
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
                            th().attr("colspan", digit_exponent_pairs.len() * 2)
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
                            .child(
                                digit_exponent_pairs
                                    .iter()
                                    .map(|(c, i)| {
                                        td().child(span().classes("red").child(match c.len() {
                                            1 => c.to_string(),
                                            _ => format!("[{c}]"),
                                        }))
                                        .child('(')
                                        .child(base())
                                        .child(sup().child(*i))
                                        .child(')')
                                    })
                                    .intersperse_with(|| td().child("+"))
                                    .collect_view(),
                            )
                            .child(filler()),
                        )
                        .child(digit_conversion)
                        .child(
                            tr().child(
                                td().classes("align-end")
                                    .child(format!("Evaluating the exponents on the base:")),
                            )
                            .child(
                                digit_exponent_pairs
                                    .iter()
                                    .map(|(c, i)| {
                                        td().child(span().child(digit_to_value(c)))
                                            .child('(')
                                            .child(
                                                span().classes("red").child(base().powi(*i as i32)),
                                            )
                                            .child(')')
                                    })
                                    .intersperse_with(|| td().child("+"))
                                    .collect_view(),
                            )
                            .child(filler()),
                        )
                        .child(
                            tr().child(td().classes("align-end").child(format!("Multiplying:")))
                                .child(
                                    digit_exponent_pairs
                                        .iter()
                                        .map(|(c, i)| {
                                            td().child(span().classes("red").child(
                                                digit_to_value(c) as f64 * base().powi(*i as i32),
                                            ))
                                        })
                                        .intersperse_with(|| td().child("+"))
                                        .collect_view(),
                                )
                                .child(filler()),
                        ),
                )
                .child(
                    tfoot().child(
                        tr().child(
                            th().classes("align-end")
                                .child(format!("Adding everything:")),
                        )
                        .child(
                            th().attr("align", "left")
                                .attr("colspan", digit_exponent_pairs.len() * 2)
                                .child(
                                    span()
                                        .classes("red")
                                        .child(
                                            digit_exponent_pairs
                                                .iter()
                                                .map(|(c, i)| {
                                                    digit_to_value(c) as f64
                                                        * base().powi(*i as i32)
                                                })
                                                .sum::<f64>(),
                                        )
                                        .child(match needs_filler {
                                            true => Some(" + ..."),
                                            false => None,
                                        }),
                                ),
                        ),
                    ),
                ),
        )
}
