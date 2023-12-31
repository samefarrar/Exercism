pub fn annotate(minefield: &[&str]) -> Vec<String> {
    if minefield.len() == 1 && minefield[0].is_empty() {
        return vec!["".to_string()];
    }
    let gap_minefield = minefield;
    let neighboring_windows = generate_neighboring_windows(gap_minefield);
    let mut new_minefield: Vec<String> = Vec::new();
    for (prev, current, next) in neighboring_windows {
        let mut new_row = String::new();
        for (index, c) in current.chars().enumerate() {
            if c == ' ' {
                let count = iterate_neighboring_window(current, index, prev, next);
                let count_character = match count {
                    0 => ' ',
                    _ => char::from_digit(count as u32, 10).unwrap()
                };
                new_row.push(count_character)
            } else {
                new_row.push(c)
            }
        }
        new_minefield.push(new_row)
    }
    return new_minefield
}

fn generate_neighboring_windows<'a>(board: &'a [&'a str]) -> Vec<(Option<&'a str>, &'a str, Option<&'a str>)> {
    let mut window_vector: Vec<(Option<&str>, &str, Option<&str>)> = Vec::new();
    for i in 0..board.len() {
        let mut prev_window = None;
        let mut next_window = None;
        if i != 0 {
            prev_window = Some(board[i - 1]);
        }
        if i != board.len() - 1 {
            next_window = Some(board[i + 1])
        }
        let current_window = board[i];
        let window: (Option<&str>, &str, Option<&str>) = (prev_window, current_window, next_window);
        window_vector.push(window)
    }
    return window_vector
}

fn iterate_neighboring_window(current: &str, index: usize, prev: Option<&str>, next: Option<&str>) -> usize {
    let start = if index > 0 {index - 1} else {index};
    let end = if index < current.len() - 1 {index + 2} else {index + 1};
    let mut count = 0;
    count = count + match prev {
        Some(prev) => prev.as_bytes()[start..end].iter().filter(|&&b| b == 42).count(),
        None => 0
    };
    count = count + match next {
        Some(next) => next.as_bytes()[start..end].iter().filter(|&&b| b == 42).count(),
        None => 0
    };
    count = count + current.as_bytes()[start..end].iter().filter(|&&b| b == 42).count();
    return count
}
