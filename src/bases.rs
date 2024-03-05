use std::collections::HashMap;

fn base_digits_to_val(digits: &str, base: f64) -> Option<f64> {
    let digit_to_val: HashMap<char, f64> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .enumerate()
        .map(|(i, c)| (c, i as f64))
        .collect();

    digits
        .chars()
        .rev()
        .enumerate()
        .fold(Some(0.0), |acc, (i, char)| {
            acc.and_then(|sum| {
                digit_to_val
                    .get(&char)
                    .and_then(|int| i.try_into().ok().map(|exp| sum + int * base.powi(exp)))
            })
        })
}

pub fn val_from_base(input: &str, base: f64) -> Option<f64> {
    match input.split('.').collect::<Vec<_>>()[..] {
        [] => Some(0.0),
        [positive] => base_digits_to_val(positive, base),
        [positive, negative] => base_digits_to_val(positive, base).and_then(|integer| {
            base_digits_to_val(negative, base).and_then(|fractional| {
                negative
                    .chars()
                    .count()
                    .try_into()
                    .ok()
                    .map(|exp| integer + fractional / (base.powi(exp)))
            })
        }),
        _ => None,
    }
}

fn digit_to_str(digit: usize) -> String {
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .get(digit..digit + 1)
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("[{digit}]"))
}

pub fn val_to_base(mut value: f64, base: f64) -> String {
    if value == 0.0 {
        return "0".to_owned();
    }
    if base == 1.0 {
        return (0..value.floor() as usize).map(|_| '1').collect();
    }

    let mut exp: i32 = (value.ln() / base.ln()).floor().max(0.0) as i32;
    let mut output = String::from("");
    let precision = -30;

    while (value.abs() > base.powi(precision) || exp >= 0) && exp > precision {
        let position = base.powi(exp);
        let digit = ((value / position) % base).floor();
        value -= digit * position;
        if exp == -1 {
            output.push('.')
        }
        output.push_str(&digit_to_str(digit as usize));
        exp -= 1;
    }

    return output;
}
