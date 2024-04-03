use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut counts = HashMap::new();

    for c in s.chars() {
        *counts.entry(c).or_insert(0) += 1;
    }

    let mut answer = 'a';
    let mut max_count = 0;
    for c in 'a'..='z' {
        if let Some(&count) = counts.get(&c) {
            if count > max_count {
                max_count = count;
                answer = c;
            }
        }
    }

    println!("{}", answer);
}
