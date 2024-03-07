use leptos::*;

#[component]
pub fn Value<F>(
    #[prop(into)] title: Signal<String>,
    value: F,
    #[prop(into)] base: Signal<f64>,
) -> impl IntoView
where
    F: 'static + Fn() -> Result<String, String>,
{
    let base_title = move || format!("Base-{}", base());
    view! {
        <div class="value">
            {move || value().map(move |v| view! {
                {title}
                <code title={base_title}>{v}<sub>{base}</sub></code>
            }).unwrap_or_else(move |e| view! {
                <>{e}</>
            })}
        </div>
    }
}
