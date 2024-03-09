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
    #[prop(into)] input_base: Signal<f64>,
    #[prop(into)] set_input_base: WriteSignal<f64>,
    #[prop(into)] output_base: Signal<f64>,
    #[prop(into)] set_output_base: WriteSignal<f64>,
) -> impl IntoView {
    move || {
        table()
            .classes("inputs")
            .child(
                thead().child(
                    tr().child(th().child("Input Value:"))
                        .child(th().child(value_in_base(string_value, (|| 10.0).into()))),
                ),
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
                                    .attr("value", input_base)
                                    .on(ev::input, update_base(set_input_base)),
                            ),
                        ),
                    )
                    .child(
                        tr().child(td().child("Output Base")).child(
                            td().child(
                                input()
                                    .attr("type", "text")
                                    .attr("value", output_base)
                                    .on(ev::input, update_base(set_output_base)),
                            ),
                        ),
                    ),
            )
    }
}

fn val_from_popular_strings(s: &str) -> Option<f64> {
    match s.to_lowercase().as_str() {
        "phi" => Some((1.0 + f64::sqrt(5.0)) / 2.0),
        "φ" => Some((1.0 + f64::sqrt(5.0)) / 2.0),
        "pi" => Some(std::f64::consts::PI),
        "e" => Some(std::f64::consts::E),
        "sqrt2" => Some(std::f64::consts::SQRT_2),
        "two" => Some(2.0),
        "binary" => Some(2.0),
        "three" => Some(3.0),
        "ternary" => Some(3.0),
        "four" => Some(4.0),
        "quaternary" => Some(4.0),
        "five" => Some(5.0),
        "quinary" => Some(5.0),
        "six" => Some(6.0),
        "senary" => Some(6.0),
        "octal" => Some(8.0),
        "eight" => Some(8.0),
        "ten" => Some(10.0),
        "decimal" => Some(10.0),
        "twelve" => Some(12.0),
        "duodecimal" => Some(12.0),
        "dozenal" => Some(12.0),
        "sixteen" => Some(16.0),
        "hex" => Some(16.0),
        "twenty" => Some(20.0),
        "vigesimal" => Some(20.0),
        "sixty" => Some(60.0),
        "sexagesimal" => Some(60.0),
        _ => None,
    }
}

fn update_base<SF>(setter: SF) -> impl Fn(Event)
where
    SF: Fn(f64),
{
    move |ev| {
        let s = event_target_value(&ev);
        match f64::from_str(&s)
            .ok()
            .or_else(|| val_from_popular_strings(&s))
        {
            Some(n) => setter(n),
            None => (),
        }
    }
}
