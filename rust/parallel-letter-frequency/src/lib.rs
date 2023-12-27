use std::collections::HashMap;
use std::thread;
use unicode_segmentation::UnicodeSegmentation;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {

    if input.is_empty() {
        return HashMap::new()
    }

    let input_owned: Vec<String> = input.iter().map(|s| s.to_string()).collect();
    let chunk_size = (input_owned.len() + worker_count - 1) / worker_count;
    let mut handles = Vec::with_capacity(worker_count);

    for chunk in input_owned.chunks(chunk_size) {
        let chunk_cloned = chunk.to_vec();
        let handle = thread::spawn(move || {
            chunk_cloned.into_iter().map(|l| count_chars_in(&l)).collect()
        });
        handles.push(handle);
    }

    let mut result = HashMap::new();
    for handle in handles {
        let partial_result: Vec<HashMap<char, usize>> = handle.join().unwrap();
        for hashmap in partial_result {
            for (key, value) in hashmap {
                let update_count = result.entry(key).or_insert(0);
                *update_count += value;
            }
        }
    }

    return result
}

fn count_chars_in(line: &str) -> HashMap<char, usize> {
    let mut counts: HashMap<char, usize> = HashMap::new();
    for g in line.graphemes(true) {
        for ch in g.to_lowercase().chars() {
            if ch.is_alphabetic() {
                let update_count = counts.entry(ch).or_insert(0);
                *update_count += 1
            }
        }
    }
    return counts;
}
