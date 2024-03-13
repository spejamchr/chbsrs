use std::collections::HashMap;

use bigdecimal::{BigDecimal, ToPrimitive};

pub fn pow(base: &BigDecimal, exp: isize) -> BigDecimal {
    match exp {
        0 => bigdecimal::One::one(),
        1 => base.clone(),
        2 => base * base,
        3 => base * base * base,
        n if n < 0 => pow(base, -exp).inverse(),
        n if n % 2 == 0 => pow(&(base * base), n / 2),
        n => base * pow(base, n - 1),
    }
}

fn floor(num: &BigDecimal) -> BigDecimal {
    num.with_scale_round(0, bigdecimal::RoundingMode::Floor)
}

fn ceil(num: &BigDecimal) -> BigDecimal {
    num.with_scale_round(0, bigdecimal::RoundingMode::Ceiling)
}

fn base_digits_to_val(digits: &str, base: &BigDecimal) -> Result<BigDecimal, String> {
    let digit_to_val: HashMap<char, BigDecimal> = "0123456789ABCDEFGHIJKLMNOPQRSTUVWXYZ"
        .chars()
        .take(ceil(base).to_usize().unwrap_or(36))
        .enumerate()
        .map(|(i, c)| (c, BigDecimal::from(i as u8)))
        .collect();

    digits
        .chars()
        .rev()
        .enumerate()
        .fold(Ok(bigdecimal::Zero::zero()), |acc, (i, char)| {
            acc.and_then(|sum| {
                digit_to_val
                    .get(&char.to_uppercase().next().unwrap_or(' '))
                    .and_then(|int| i.try_into().ok().map(|exp| sum + int * pow(&base, exp)))
                    .ok_or_else(|| format!("Unrecognized digit in input: {char}"))
            })
        })
}

pub fn val_from_base(input: &str, base: &BigDecimal) -> Result<BigDecimal, String> {
    if base <= &bigdecimal::One::one() {
        return Err("Input base must be greater than 1".to_string());
    }
    match input.split('.').collect::<Vec<_>>()[..] {
        [] => Ok(bigdecimal::Zero::zero()),
        [positive] => base_digits_to_val(positive, base),
        [positive, negative] => base_digits_to_val(positive, base).and_then(|integer| {
            base_digits_to_val(negative, base).and_then(|fractional| {
                negative
                    .chars()
                    .count()
                    .try_into()
                    .map(|exp| integer + fractional / (pow(base, exp)))
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

pub fn val_to_base(value: &BigDecimal, base: &BigDecimal) -> Result<String, String> {
    let mut value = value.clone();
    if base <= &bigdecimal::One::one() {
        return Err("Output base must be greater than 1".to_string());
    }
    if value == bigdecimal::Zero::zero() {
        return Ok("0".to_owned());
    }

    let mut exp = 0;
    while pow(base, exp) < value {
        exp += 1;
    }
    while exp > bigdecimal::Zero::zero()
        && floor(&((value.clone() / pow(base, exp)) % base)) == bigdecimal::Zero::zero()
    {
        exp -= 1;
    }
    let mut output = String::from("");
    let precision = -8;
    let most_precise = pow(base, precision * 2);

    while (value.abs() > most_precise || exp >= 0) && exp >= precision {
        if exp == precision {
            output.push_str("…"); // ellide
            return Ok(output);
        }
        let position = pow(base, exp);
        let digit = floor(&((value.clone() / position.clone()) % base));
        value -= digit.clone() * position.clone();
        if exp == -1 {
            output.push('.')
        }
        let dusize = digit.to_usize().unwrap();
        output.push_str(&digit_to_str(dusize));
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

#[cfg(test)]
mod tests {
    use std::str::FromStr;

    use super::*;

    #[test]
    fn base10_conversion() {
        let decimal = val_from_base("12345", &BigDecimal::from(10)).unwrap();
        assert_eq!(
            BigDecimal::from(12345).round(20).to_string(),
            decimal.round(20).to_string()
        );
    }

    #[test]
    fn fails_with_base1() {
        let decimal = val_from_base("12345", &BigDecimal::from(1));
        assert!(decimal.is_err());
    }

    #[test]
    fn fails_with_multiple_periods() {
        let decimal = val_from_base("12.34.5", &BigDecimal::from(10));
        assert!(decimal.is_err());
    }

    #[test]
    fn parses_1_plus_sqrt2_from_base_sqrt2() {
        let decimal = val_from_base("11", &BigDecimal::from(2).sqrt().unwrap());
        assert_eq!(
            (BigDecimal::from(2).sqrt().unwrap() + 1_u32)
                .round(20)
                .to_string(),
            decimal.unwrap().round(20).to_string()
        );
    }

    #[test]
    fn parses_3_from_base_sqrt2() {
        let decimal = val_from_base("101", &BigDecimal::from(2).sqrt().unwrap());
        assert_eq!(
            (BigDecimal::from(3)).round(20).to_string(),
            decimal.unwrap().round(20).to_string()
        );
    }

    #[test]
    fn show_2_in_base_10() {
        let string = val_to_base(&BigDecimal::from(2), &BigDecimal::from(10));
        assert_eq!(Ok("2".to_owned()), string);
    }

    #[test]
    fn show_small_value_in_base_10() {
        let string = val_to_base(
            &BigDecimal::from_str("0.0000001").unwrap(),
            &BigDecimal::from(10),
        );
        assert_eq!(Ok("0.0000001".to_owned()), string);
    }

    #[test]
    fn elide_smaller_value_in_base_10() {
        let string = val_to_base(
            &BigDecimal::from_str("0.00000001").unwrap(),
            &BigDecimal::from(10),
        );
        assert_eq!(Ok("0.0000000…".to_owned()), string);
    }

    #[test]
    fn round_parsing_correctly() {
        let string = val_to_base(&BigDecimal::from(3), &BigDecimal::from_str("10.3").unwrap());
        assert_eq!(Ok("3".to_owned()), string);
    }
}
