use std::str::FromStr;

use bigdecimal::BigDecimal;
use leptos::*;

use crate::{
    bases::{val_from_base, val_to_base},
    components::{home_inputs::HomeInputs, output_details::OutputDetails},
};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (input_string, set_input_string) = create_signal("23982982383829823983".to_string());
    let (input_base, set_input_base) = create_signal(BigDecimal::from_str("10.3").unwrap());
    let (output_base, set_output_base) = create_signal(BigDecimal::from_str("10.3").unwrap());

    let result_value = create_memo(move |_| val_from_base(&input_string(), &input_base()));

    let string_value =
        create_memo(move |_| result_value().and_then(|v| val_to_base(&v, &BigDecimal::from(10))));
    let output_representation = create_memo(move |_| {
        result_value()
            .map_err(|_| "".to_string())
            .and_then(|v| val_to_base(&v, &output_base()))
    });

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

                <HomeInputs
                    string_value=string_value
                    output_representation=output_representation
                    input_string=input_string
                    set_input_string=set_input_string
                    input_base=input_base
                    set_input_base=set_input_base
                    output_base=output_base
                    set_output_base=set_output_base
                />

                <OutputDetails output={output_representation} base={output_base}/>

            </div>
        </ErrorBoundary>
    }
}
