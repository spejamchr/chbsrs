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

pub fn rep_to_digit_exponent_pairs(rep: &str) -> Vec<(String, isize)> {
    let mut digits: Vec<String> = Vec::new();
    let chars: Vec<char> = rep.chars().collect();
    let mut idx = 0;
    while idx < chars.len() {
        if chars[idx] == '[' {
            let mut j = idx + 1;
            while j < chars.len() && chars[j] != ']' {
                j += 1
            }
            digits.push(chars[idx + 1..j].iter().collect());
            idx = j + 1;
        } else {
            digits.push(chars[idx].to_string());
            idx += 1;
        }
    }

    let max_exp: isize = (digits.iter().take_while(|&c| c != ".").count() - 1)
        .try_into()
        .unwrap();

    digits
        .into_iter()
        .filter(|c| c != ".")
        .zip((-max_exp..).map(|i| -i))
        .collect()
}
