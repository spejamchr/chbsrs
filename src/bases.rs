use std::collections::HashMap;

fn base_digits_to_val(digits: &str, base: f64) -> Result<f64, String> {
    let digit_to_val: HashMap<char, f64> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .take(base.ceil() as usize)
        .enumerate()
        .map(|(i, c)| (c, i as f64))
        .collect();

    digits
        .chars()
        .rev()
        .enumerate()
        .fold(Ok(0.0), |acc, (i, char)| {
            acc.and_then(|sum| {
                digit_to_val
                    .get(&char.to_uppercase().next().unwrap_or(' '))
                    .and_then(|int| i.try_into().ok().map(|exp| sum + int * base.powi(exp)))
                    .ok_or_else(|| format!("Unrecognized digit in input: {char}"))
            })
        })
}

pub fn val_from_base(input: &str, base: f64) -> Result<f64, String> {
    if base <= 1.0 {
        return Err("Input base must be greater than 1".to_string());
    }
    match input.split('.').collect::<Vec<_>>()[..] {
        [] => Ok(0.0),
        [positive] => base_digits_to_val(positive, base),
        [positive, negative] => base_digits_to_val(positive, base).and_then(|integer| {
            base_digits_to_val(negative, base).and_then(|fractional| {
                negative
                    .chars()
                    .count()
                    .try_into()
                    .map(|exp| integer + fractional / (base.powi(exp)))
                    .map_err(|e| e.to_string())
            })
        }),
        _ => Err("The input may have at most one `.`".to_string()),
    }
}

fn digit_to_str(digit: usize) -> String {
    "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .get(digit..digit + 1)
        .map(|s| s.to_string())
        .unwrap_or_else(|| format!("[{digit}]"))
}

pub fn val_to_base(mut value: f64, base: f64) -> Result<String, String> {
    if base <= 1.0 {
        return Err("Output base must be greater than 1".to_string());
    }
    if value == 0.0 {
        return Ok("0".to_owned());
    }

    let mut exp: i32 = (value.ln() / base.ln()).floor().max(0.0) as i32;
    let mut output = String::from("");
    let precision = -10;

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

    return Ok(output);
}
