fn main() {
    // * LeetCode Question: 8. String to Integer (atoi)
    // * https://leetcode.com/problems/string-to-integer-atoi/description/
    my_atoi_my_answer("s".to_string());
    my_atoi_clean_answer("s".to_string());
}

fn my_atoi_my_answer(s: String) -> i32 {
    let mut chars = s.trim_start().chars();
    let mut new_num = match chars.next() {
        Some(x) if x == '+' => '+',
        Some(x) if x == '-' => '-',
        Some(no_sign) if no_sign.is_numeric() => no_sign,
        _ => return 0,
    }
    .to_string();

    for ch in chars {
        if !ch.is_numeric() {
            break;
        }
        new_num.push(ch);
    }

    match new_num.parse::<i32>() {
        Ok(num) => num,
        Err(e) => match e.kind() {
            std::num::IntErrorKind::PosOverflow => i32::MAX,
            std::num::IntErrorKind::NegOverflow => i32::MIN,
            _ => 0,
        },
    }
}

fn my_atoi_clean_answer(s: String) -> i32 {
    let s = s.trim_start();
    let (s, sign) = match s.strip_prefix('-') {
        Some(s) => (s, -1),
        None => (s.strip_prefix('+').unwrap_or(s), 1),
    };
    s.chars()
        .map(|c| c.to_digit(10))
        .take_while(Option::is_some)
        .flatten()
        .fold(0, |acc, digit| {
            acc.saturating_mul(10).saturating_add(sign * digit as i32)
        })
}
