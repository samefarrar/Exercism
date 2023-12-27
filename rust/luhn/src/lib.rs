pub fn is_valid(code: &str) -> bool {
    if !code.chars().all(is_digit_or_space) {
        return false
    }

    let code: Vec<u32> = code.chars()
                             .filter(|c| c.is_digit(10))
                             .rev()
                             .map(|c| c.to_digit(10).unwrap())
                             .collect();

    if code.len() <= 1 {
        return false;
    }

    let acc = code.into_iter().enumerate().fold(0, |acc, (i, val)| {
        let processed_val = if i % 2 != 0 { val * 2 } else { val };
        let processed_val = if processed_val > 9 { processed_val - 9 } else { processed_val };
        acc + processed_val
    });

    acc % 10 == 0
}

fn is_digit_or_space(c: char) -> bool {
    if c == ' '|| c.is_numeric() {
        return true
    } else {
        return false
    }
}
