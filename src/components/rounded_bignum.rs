use bigdecimal::BigDecimal;
use leptos::{html::*, *};
use std::num::NonZeroU64;

use crate::bases::rounded_string;

pub fn rounded_bignum(num: BigDecimal, hard_limit: Option<NonZeroU64>) -> impl IntoView {
    let rounded = rounded_string(&num, hard_limit);
    span().child(rounded).attr("title", num.to_string())
}
