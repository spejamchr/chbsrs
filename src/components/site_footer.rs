use std::time::Duration;

use leptos::{html::*, *};
use web_sys::js_sys::Date;

static EMOJI: [char; 43] = [
    '🙃', '🚘', '🌔', '🐐', '💥', '🍓', '🚉', '🐰', '🐻', '🦏', '😂', '🌜', '🐛', '🐼', '🌙', '🚙',
    '🦔', '🐝', '🚦', '🍓', '🐪', '👹', '🍱', '🛹', '🛴', '🚑', '🐢', '🐗', '🌽', '🍜', '🦞', '😽',
    '🚂', '🥀', '🐮', '🐱', '😃', '🌞', '🍪', '🦆', '🐯', '🧐', '😴',
];

pub fn site_footer() -> impl IntoView {
    let page_loaded_at = Date::now();
    let index_step_size = (page_loaded_at % (EMOJI.len() as f64 - 1.0)) as usize + 1;
    let (index, set_index) = create_signal(index_step_size);
    let (found_bunny_at, set_found_bunny_at) = create_signal::<Option<f64>>(None);
    set_interval(
        move || set_index.update(|n| *n = (*n + index_step_size) % EMOJI.len()),
        Duration::from_secs(42),
    );
    (
        span()
            .classes("site-footer")
            .child(
                span().child("made by ").child(
                    a().attr("href", "https://github.com/spejamchr")
                        .child("spejamchr"),
                ),
            )
            .child(
                code()
                    .id("footer-emoji")
                    .attr("tabindex", "0")
                    .attr("title", move || match found_bunny_at() {
                        Some(_) => "you found the easter bunny... ↓",
                        None => "there are no easter eggs here",
                    })
                    .on(ev::click, move |_| {
                        if EMOJI[index()] == '🐰' && found_bunny_at().is_none() {
                            set_found_bunny_at(Some(Date::now()));
                        }
                    })
                    .child(move || EMOJI[index()]),
            )
            .child(
                span().child("source on ").child(
                    a().attr("href", "https://github.com/spejamchr/chbsrs/")
                        .child("github"),
                ),
            ),
        move || {
            found_bunny_at().map(|n| {
                pre()
                    .attr(
                        "title",
                        format!(
                            "found easter eggs after {} seconds",
                            (n - page_loaded_at) / 1000.0
                        ),
                    )
                    .child("🐇 🧺🥚 🥚🐣  🍫")
            })
        },
    )
}
