use std::collections::HashMap;
use std::io::BufRead;

fn main() {
    let mut words: HashMap<String, u64> = HashMap::new();
    let mut line = String::new();
    let mut xin = std::io::stdin().lock();

    loop {
        line.clear();
        match xin.read_line(&mut line) {
            Ok(0) => break,
            Ok(_c) => {
                for word in line.split_whitespace() {
                    match words.get_mut(word) {
                        Some(c) => *c += 1,
                        None => {
                            words.insert(word.to_string(), 1);
                            ()
                        }
                    }
                }
            }
            _ => break,
        }
    }

    for (word, count) in words {
        println!("{} {}", word, count);
    }
}
