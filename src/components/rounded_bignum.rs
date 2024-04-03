use bigdecimal::BigDecimal;
use leptos::{html::*, *};
use std::num::NonZeroU64;

use crate::bases::rounded_string;

pub fn rounded_bignum(num: BigDecimal, hard_limit: Option<NonZeroU64>) -> impl IntoView {
    let rounded = rounded_string(&num, hard_limit);
    let s = span().child(&rounded);
    if rounded != num.to_string() {
        return s.attr("tabindex", "0").attr("title", num.to_string());
    }
    s
}
