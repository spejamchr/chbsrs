use bigdecimal::BigDecimal;
use core::str::FromStr;
use leptos::{html::*, *};
use web_sys::Event;

use crate::components::value_in_base::value_in_base;

#[component]
pub fn HomeInputs(
    string_value: Memo<Result<String, String>>,
    output_representation: Memo<Result<String, String>>,
    #[prop(into)] input_string: Signal<String>,
    #[prop(into)] set_input_string: WriteSignal<String>,
    #[prop(into)] input_base: Signal<BigDecimal>,
    #[prop(into)] set_input_base: WriteSignal<BigDecimal>,
    #[prop(into)] output_base: Signal<BigDecimal>,
    #[prop(into)] set_output_base: WriteSignal<BigDecimal>,
) -> impl IntoView {
    move || {
        table()
            .classes("inputs")
            .child(
                thead().child(tr().child(th().child("Input Value:")).child(th().child(
                    value_in_base(string_value, (|| BigDecimal::from(10)).into()),
                ))),
            )
            .child(
                tfoot().child(
                    tr().child(th().child("Output Value:"))
                        .child(th().child(value_in_base(output_representation, output_base))),
                ),
            )
            .child(
                tbody()
                    .child(
                        tr().child(
                            td().child(label().attr("for", "InputString").child("Input String")),
                        )
                        .child(
                            td().child(
                                input()
                                    .id("InputString")
                                    .style("width", move || {
                                        format!("{}ch", input_string().chars().count().max(18) + 2)
                                    })
                                    .attr("type", "text")
                                    .attr("value", input_string)
                                    .on(ev::input, move |ev| {
                                        set_input_string(event_target_value(&ev))
                                    }),
                            ),
                        ),
                    )
                    .child(
                        tr().child(td().child("Input Base")).child(
                            td().child(
                                input()
                                    .attr("type", "text")
                                    .attr("value", move || input_base().to_string())
                                    .on(ev::input, update_base(set_input_base)),
                            ),
                        ),
                    )
                    .child(
                        tr().child(td().child("Output Base")).child(
                            td().child(
                                input()
                                    .attr("type", "text")
                                    .attr("value", move || output_base().to_string())
                                    .on(ev::input, update_base(set_output_base)),
                            ),
                        ),
                    ),
            )
    }
}

fn val_from_popular_strings(s: &str) -> Option<BigDecimal> {
    match s.to_lowercase().as_str() {
        "phi" => Some((BigDecimal::from(5).sqrt().unwrap() + 1) / 2),
        "φ" => Some((BigDecimal::from(5).sqrt().unwrap() + 1) / 2),
        "pi" => Some(
            BigDecimal::from_str("3.14159265358979323846264338327950288419716939937510").unwrap(),
        ),
        "π" => Some(
            BigDecimal::from_str("3.14159265358979323846264338327950288419716939937510").unwrap(),
        ),
        "e" => Some(
            BigDecimal::from_str("2.71828182845904523536028747135266249775724709369995").unwrap(),
        ),
        "sqrt2" => Some(BigDecimal::from(2).sqrt().unwrap()),
        "two" => Some(BigDecimal::from(2)),
        "binary" => Some(BigDecimal::from(2)),
        "three" => Some(BigDecimal::from(3)),
        "ternary" => Some(BigDecimal::from(3)),
        "four" => Some(BigDecimal::from(4)),
        "quaternary" => Some(BigDecimal::from(4)),
        "five" => Some(BigDecimal::from(5)),
        "quinary" => Some(BigDecimal::from(5)),
        "six" => Some(BigDecimal::from(6)),
        "senary" => Some(BigDecimal::from(6)),
        "octal" => Some(BigDecimal::from(8)),
        "eight" => Some(BigDecimal::from(8)),
        "ten" => Some(BigDecimal::from(10)),
        "decimal" => Some(BigDecimal::from(10)),
        "twelve" => Some(BigDecimal::from(12)),
        "duodecimal" => Some(BigDecimal::from(12)),
        "dozenal" => Some(BigDecimal::from(12)),
        "sixteen" => Some(BigDecimal::from(16)),
        "hex" => Some(BigDecimal::from(16)),
        "twenty" => Some(BigDecimal::from(20)),
        "vigesimal" => Some(BigDecimal::from(20)),
        "sixty" => Some(BigDecimal::from(60)),
        "sexagesimal" => Some(BigDecimal::from(60)),
        _ => None,
    }
}

fn update_base<SF>(setter: SF) -> impl Fn(Event)
where
    SF: Fn(BigDecimal),
{
    move |ev| {
        let s = event_target_value(&ev);
        if let Some(n) = BigDecimal::from_str(&s)
            .ok()
            .or_else(|| val_from_popular_strings(&s))
        {
            setter(n)
        }
    }
}
