use std::collections::HashMap;
use std::sync::{Arc, Mutex};
use std::thread;

pub fn frequency(input: &[&str], worker_count: usize) -> HashMap<char, usize> {
    if input.is_empty() {
        return HashMap::new();
    }

    let input_owned: Vec<String> = input.iter().map(|s| s.to_string()).collect();

    let chunk_size = (input.len() + worker_count - 1) / worker_count;
    let result = Arc::new(Mutex::new(HashMap::new()));
    let mut handles = Vec::new();

    for chunk in input_owned.chunks(chunk_size) {
        let result = Arc::clone(&result);
        let chunk = chunk.to_vec();

        let handle = thread::spawn(move || {
            let mut local_map = HashMap::new();
            for line in chunk {
                for ch in line.chars().filter(|c| c.is_alphabetic()).map(|c| c.to_lowercase().next().unwrap()) {
                    *local_map.entry(ch).or_insert(0) += 1;
                }
            }

            let mut result = result.lock().unwrap();
            for (key, value) in local_map {
                *result.entry(key).or_insert(0) += value;
            }
        });
        handles.push(handle);
    }

    for handle in handles {
        handle.join().expect("Thread panicked");
    }

    Arc::try_unwrap(result).unwrap().into_inner().unwrap()
}
