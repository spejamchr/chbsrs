use leptos::{html::*, *};

use crate::{
    bases::BaseConversion,
    components::{
        home_inputs::HomeInputs, output_details::OutputDetails, site_footer::site_footer,
    },
};

/// Default Home Page
#[component]
pub fn Home() -> impl IntoView {
    let (input_string, set_input_string) = create_signal("123.45".to_string());
    let (input_base_string, set_input_base_string) = create_signal(String::from("10"));
    let (output_base_string, set_output_base_string) = create_signal(String::from("π"));

    let base_conversion = create_memo::<BaseConversion>(move |prev| {
        BaseConversion::new_with_defaults(
            input_string(),
            input_base_string(),
            output_base_string(),
            prev,
        )
    });

    let also_try = sub()
        .child("Also try bases: ")
        .child(code().child("pi"))
        .child(", ")
        .child(code().child("e"))
        .child(", ")
        .child(code().child("sqrt2"))
        .child(", ")
        .child(code().child("phi"))
        .child(".");

    let footer = site_footer();

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
                    base_conversion=base_conversion
                    set_input_string=set_input_string
                    set_input_base_string=set_input_base_string
                    set_output_base_string=set_output_base_string
                />

                {also_try}

                <OutputDetails base_conversion=base_conversion />

                {footer}

            </div>
        </ErrorBoundary>
    }
}
