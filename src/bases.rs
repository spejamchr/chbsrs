use std::{num::NonZeroU64, str::FromStr};

use bigdecimal::{BigDecimal, ToPrimitive};

#[derive(PartialEq, Clone)]
pub struct BaseConversion {
    pub input_string: String,
    pub input_base: BigDecimal,
    pub input_base_string: String,
    pub output_base: BigDecimal,
    pub output_base_string: String,
}

static FALLBACK_INPUT_BASE: i32 = 10;
static FALLBACK_OUTPUT_BASE: i32 = 10;

impl BaseConversion {
    pub fn new_with_defaults(
        input_string: String,
        input_base_string: String,
        output_base_string: String,
        base_conversion: Option<&Self>,
    ) -> Self {
        Self {
            input_string,
            input_base: BigDecimal::from_str(&input_base_string)
                .or_else(|e| val_from_popular_strings(&input_base_string).ok_or(e))
                .or_else(|e| base_conversion.map(|c| c.input_base.clone()).ok_or(e))
                .unwrap_or_else(|_| BigDecimal::from(FALLBACK_INPUT_BASE)),
            input_base_string,
            output_base: BigDecimal::from_str(&output_base_string)
                .or_else(|e| val_from_popular_strings(&output_base_string).ok_or(e))
                .or_else(|e| base_conversion.map(|c| c.output_base.clone()).ok_or(e))
                .unwrap_or_else(|_| BigDecimal::from(FALLBACK_OUTPUT_BASE)),
            output_base_string,
        }
    }

    fn base_10_value(&self) -> Result<BigDecimal, String> {
        val_from_base(&self.input_string, &self.input_base)
    }

    pub fn base_10_string(&self) -> Result<String, String> {
        self.base_10_value()
            .and_then(|v| val_to_base(&v, &BigDecimal::from(10)))
    }

    pub fn output_string(&self) -> Result<String, String> {
        self.base_10_value()
            .and_then(|v| val_to_base(&v, &self.output_base))
    }
}

fn val_from_popular_strings(s: &str) -> Option<BigDecimal> {
    match s.to_lowercase().as_str() {
        "phi" => Some((BigDecimal::from(5).sqrt().unwrap() + 1) / 2),
        "φ" => Some((BigDecimal::from(5).sqrt().unwrap() + 1) / 2),
        "pi" => Some(
            BigDecimal::from_str("3.14159265358979323846264338327950288419716939937510").unwrap(),
        ),
        "π" => Some(
            BigDecimal::from_str("3.14159265358979323846264338327950288419716939937510").unwrap(),
        ),
        "e" => Some(
            BigDecimal::from_str("2.71828182845904523536028747135266249775724709369995").unwrap(),
        ),
        "sqrt2" => Some(BigDecimal::from(2).sqrt().unwrap()),
        "two" => Some(BigDecimal::from(2)),
        "binary" => Some(BigDecimal::from(2)),
        "three" => Some(BigDecimal::from(3)),
        "ternary" => Some(BigDecimal::from(3)),
        "four" => Some(BigDecimal::from(4)),
        "quaternary" => Some(BigDecimal::from(4)),
        "five" => Some(BigDecimal::from(5)),
        "quinary" => Some(BigDecimal::from(5)),
        "six" => Some(BigDecimal::from(6)),
        "senary" => Some(BigDecimal::from(6)),
        "octal" => Some(BigDecimal::from(8)),
        "eight" => Some(BigDecimal::from(8)),
        "ten" => Some(BigDecimal::from(10)),
        "decimal" => Some(BigDecimal::from(10)),
        "twelve" => Some(BigDecimal::from(12)),
        "duodecimal" => Some(BigDecimal::from(12)),
        "dozenal" => Some(BigDecimal::from(12)),
        "sixteen" => Some(BigDecimal::from(16)),
        "hex" => Some(BigDecimal::from(16)),
        "twenty" => Some(BigDecimal::from(20)),
        "vigesimal" => Some(BigDecimal::from(20)),
        "sixty" => Some(BigDecimal::from(60)),
        "sexagesimal" => Some(BigDecimal::from(60)),
        _ => None,
    }
}

pub fn rounded_string(num: BigDecimal, hard_limit: Option<NonZeroU64>) -> String {
    if let Some(hl) = hard_limit {
        if num.digits() > hl.get() {
            let str = num
                .with_precision_round(hl, bigdecimal::RoundingMode::Down)
                .to_string();
            if str.split('E').count() > 1 {
                return str;
            } else {
                return format!("{}…", str); // ellide
            }
        }
    }
    let limit = 8;
    let (_, scale) = num.as_bigint_and_exponent();
    if scale > limit {
        format!("{}…", num.with_scale(limit)) // ellide
    } else if num.is_integer() {
        num.to_u32()
            .map(|n| n.to_string())
            .unwrap_or_else(|| num.to_string())
    } else {
        num.to_f64()
            .map(|n| n.to_string())
            .unwrap_or_else(|| num.to_string())
    }
}

pub fn pow(base: &BigDecimal, exp: isize) -> BigDecimal {
    match exp {
        0 => bigdecimal::One::one(),
        1 => base.clone(),
        2 => base.square().round(50).normalized(),
        3 => base * base.square().round(50).normalized(),
        n if n < 0 => pow(base, -exp).inverse(),
        n if n % 2 == 0 => pow(&(base.square().round(50).normalized()), n / 2),
        n => base * pow(base, n - 1),
    }
}

fn floor(num: &BigDecimal) -> BigDecimal {
    num.with_scale_round(0, bigdecimal::RoundingMode::Floor)
}

fn base_digits_to_val(digits: &str, base: &BigDecimal) -> Result<BigDecimal, String> {
    let mut power = base.inverse();
    rep_to_digit_exponent_pairs(digits)
        .into_iter()
        .rev()
        .try_fold(bigdecimal::Zero::zero(), |sum: BigDecimal, (char, _)| {
            power *= base;
            match char.parse().or_else(|_| u32::from_str_radix(&char, 36)) {
                Ok(int) => Ok(sum + int * power.clone()),
                Err(_) => Err(format!("Unrecognized digit in input: {char}")),
            }
        })
        .map(|n| n.round(32).normalized())
}

fn val_from_base(input: &str, base: &BigDecimal) -> Result<BigDecimal, String> {
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

fn digit_to_string(digit: usize) -> String {
    static DIGITS: [&str; 36] = [
        "0", "1", "2", "3", "4", "5", "6", "7", "8", "9", "A", "B", "C", "D", "E", "F", "G", "H",
        "I", "J", "K", "L", "M", "N", "O", "P", "Q", "R", "S", "T", "U", "V", "W", "X", "Y", "Z",
    ];
    match DIGITS.get(digit) {
        Some(s) => s.to_string(),
        None => format!("[{digit}]"),
    }
}

fn val_to_base(value: &BigDecimal, base: &BigDecimal) -> Result<String, String> {
    let mut value = value.clone();
    if base <= &bigdecimal::One::one() {
        return Err("Output base must be greater than 1".to_string());
    }
    if value == bigdecimal::Zero::zero() {
        return Ok("0".to_owned());
    }

    let mut exp = 0;
    let mut power = BigDecimal::from(1);
    while power < value {
        exp += 1;
        power *= base;
    }
    while exp > bigdecimal::Zero::zero()
        && floor(&((value.clone() / power.clone()) % base)) == bigdecimal::Zero::zero()
    {
        exp -= 1;
        power = power / base;
    }
    let mut output = String::from("");
    let precision = -9;
    let most_precise = pow(base, precision * 2);

    while (value.abs() > most_precise || exp >= 0) && exp >= precision {
        if exp == precision {
            output.push('…'); // ellide
            return Ok(output);
        }
        let digit = floor(&((value.clone() / power.clone()) % base));
        value -= digit.clone() * power.clone();
        if exp == -1 {
            output.push('.')
        }
        let dusize = digit.to_usize().unwrap();
        output.push_str(&digit_to_string(dusize));
        exp -= 1;
        power = power / base;
    }

    Ok(output)
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

    let max_exp: isize =
        <usize as TryInto<isize>>::try_into(digits.iter().take_while(|&c| c != ".").count())
            .unwrap()
            - 1;

    digits
        .into_iter()
        .filter(|c| c != ".")
        .zip((-max_exp..).map(|i| -i))
        .collect()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn base10_conversion() {
        let decimal = val_from_base("12345", &BigDecimal::from(10)).unwrap();
        assert_eq!(BigDecimal::from(12345).to_string(), decimal.to_string());
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
    fn parses_3_from_base_10_3() {
        let decimal = val_from_base("3", &BigDecimal::from_str("10.3").unwrap());
        assert_eq!(
            (BigDecimal::from(3)).to_string(),
            decimal.unwrap().to_string()
        );
    }

    #[test]
    fn parses_from_base_100() {
        let decimal = val_from_base("[99]", &BigDecimal::from(100));
        assert_eq!(
            (BigDecimal::from(99)).to_string(),
            decimal.unwrap().to_string()
        );
    }

    #[test]
    fn parses_decimal() {
        let decimal = val_from_base("0.12345678", &BigDecimal::from_str("10").unwrap());
        assert_eq!("0.12345678".to_string(), decimal.unwrap().to_string());
    }

    #[test]
    fn parses_decimal_without_leading_zero() {
        let decimal = val_from_base(".1", &BigDecimal::from_str("10").unwrap());
        assert_eq!("0.1".to_string(), decimal.unwrap().to_string());
    }

    #[test]
    fn round_small_decimal() {
        let decimal = BigDecimal::from_str("0.12345678")
            .map(|v| rounded_string(v, None))
            .unwrap();
        assert_eq!("0.12345678".to_string(), decimal);
    }

    #[test]
    fn round_longer_decimal() {
        let decimal = BigDecimal::from_str("0.123456789")
            .map(|v| rounded_string(v, None))
            .unwrap();
        assert_eq!("0.12345678…".to_string(), decimal);
    }

    #[test]
    fn round_large_integer() {
        let decimal = rounded_string(pow(&BigDecimal::from(10), 10), NonZeroU64::new(8));
        assert_eq!("1E+10".to_string(), decimal);
    }

    #[test]
    fn round_1234567890123_with_limit() {
        let decimal = rounded_string(
            BigDecimal::from_str("1234567890123").unwrap(),
            NonZeroU64::new(8),
        );
        assert_eq!("1.2345678E+12".to_string(), decimal);
    }

    #[test]
    fn round_pi() {
        let decimal = rounded_string(
            BigDecimal::from_str("3.14159265358979323846264338327950288419716939937510").unwrap(),
            NonZeroU64::new(8),
        );
        assert_eq!("3.1415926…".to_string(), decimal);
    }

    #[test]
    fn round_fake_pi() {
        let decimal = rounded_string(BigDecimal::from_str("3.14").unwrap(), NonZeroU64::new(8));
        assert_eq!("3.14".to_string(), decimal);
    }

    #[test]
    fn round_small_integer() {
        let decimal = rounded_string(pow(&BigDecimal::from(10), 8), NonZeroU64::new(8));
        assert_eq!("100000000".to_string(), decimal);
    }

    #[test]
    fn show_2_in_base_10() {
        let string = val_to_base(&BigDecimal::from(2), &BigDecimal::from(10));
        assert_eq!(Ok("2".to_owned()), string);
    }

    #[test]
    fn show_small_value_in_base_10() {
        let string = val_to_base(
            &BigDecimal::from_str("0.00000001").unwrap(),
            &BigDecimal::from(10),
        );
        assert_eq!(Ok("0.00000001".to_owned()), string);
    }

    #[test]
    fn elide_smaller_value_in_base_10() {
        let string = val_to_base(
            &BigDecimal::from_str("0.000000001").unwrap(),
            &BigDecimal::from(10),
        );
        assert_eq!(Ok("0.00000000…".to_owned()), string);
    }

    #[test]
    fn round_parsing_correctly() {
        let string = val_to_base(&BigDecimal::from(3), &BigDecimal::from_str("10.3").unwrap());
        assert_eq!(Ok("3".to_owned()), string);
    }
}
