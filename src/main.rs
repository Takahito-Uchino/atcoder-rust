use std::collections::HashSet;

use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut s_set = HashSet::new();

    for i in 0..s.len() {
        for j in i..s.len() {
            let slice = &s[i..j + 1];
            s_set.insert(slice);
        }
    }

    println!("{}", s_set.len())
}
