pub fn convert_localized_string(s: &str) -> String {
    s.replace(',', ".")
}

pub fn celsius_to_fahrenheit(c: i64) -> i64 {
    (c as f64 * (9f64 / 5f64)) as i64 + 32
}
