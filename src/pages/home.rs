use core::str::FromStr;
use leptos::*;

use crate::{
    bases::{val_from_base, val_to_base},
    components::{output_details::OutputDetails, value::Value},
};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (input_string, set_input_string) = create_signal("11EE4E7C6FF3A6".to_string());
    let (input_base, set_input_base) = create_signal(16.0);
    let (output_base, set_output_base) = create_signal(42.0);

    let val_from_popular_strings = move |s: &str| match s.to_lowercase().as_str() {
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
    };

    let update_base = move |setter: WriteSignal<f64>| {
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
    };

    let result_value = move || val_from_base(&input_string(), input_base());
    let string_value = move || result_value().map(|v| v.to_string());
    let output_representation = move || {
        result_value()
            .map_err(|_| "".to_string())
            .and_then(|v| val_to_base(v, output_base()))
    };

    view! {
        <ErrorBoundary fallback=|errors| {
            view! {
                <h1>"Uh oh! Something went wrong!"</h1>

                <p>"Errors: "</p>
                // Render a list of errors as strings - good for development purposes
                <ul>
                    {move || {
                        errors
                            .get()
                            .into_iter()
                            .map(|(_, e)| view! { <li>{e.to_string()}</li> })
                            .collect_view()
                    }}

                </ul>
            }
        }>

            <div class="container">

                <h1>"ChangeBase"</h1>

                <Value
                    title={move || "Input Value: ".to_string()}
                    value={string_value}
                    base={|| 10.0}
                />

                <div class="inputs">
                    <label>
                    "Input String"
                        <input type="text" value=input_string on:input= move |ev| {
                            set_input_string(event_target_value(&ev))
                        }/>
                    </label>
                    <label>
                    "Input Base"
                        <input value=input_base on:input=update_base(set_input_base) />
                    </label>
                    <label>
                    "Output Base"
                        <input value=output_base on:input=update_base(set_output_base) />
                    </label>
                </div>


                <Value
                    title={move || "Output Value: ".to_string()}
                    value={output_representation}
                    base={output_base}
                />

                <OutputDetails output={output_representation} base={output_base}/>

            </div>
        </ErrorBoundary>
    }
}
