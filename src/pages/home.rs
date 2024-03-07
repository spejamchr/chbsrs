use core::str::FromStr;
use leptos::*;

use crate::{
    bases::{val_from_base, val_to_base},
    components::value::Value,
};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (input_string, set_input_string) = create_signal("0".to_string());
    let (input_base, set_input_base) = create_signal(10.0);
    let (output_base, set_output_base) = create_signal(2.0);

    let value = move || val_from_base(&input_string(), input_base());
    let string_value = move || value().map(|v| v.to_string());
    let output_representation = move || {
        value()
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

                <h1>"Change.Base"</h1>

                <Value
                    title={move || "Input Value: ".to_string()}
                    value={string_value}
                    base={input_base}
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
                        <input type="number" value=input_base on:input= move |ev| {
                            if let Ok(n) = f64::from_str(&event_target_value(&ev)) {
                                set_input_base(n);
                            }
                        }/>
                        </label>
                    <label>
                    "Output Base"
                        <input type="number" value=output_base on:input= move |ev| {
                            if let Ok(n) = f64::from_str(&event_target_value(&ev)) {
                                set_output_base(n);
                            }
                        }/>
                    </label>
                </div>


                <Value
                    title={move || "Output Value: ".to_string()}
                    value={output_representation}
                    base={output_base}
                />

            </div>
        </ErrorBoundary>
    }
}
