use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let mut lengths = Vec::new();

    lengths.push(1);

    for i in 0..s.len() - 1 {
        for j in i + 1..s.len() {
            let mut is_rotate = true;
            for k in i..=j {
                if s[k] != s[j - (k - i)] {
                    is_rotate = false;
                }
            }
            if is_rotate {
                lengths.push(j - i + 1);
            }
        }
    }

    println!("{}", lengths.iter().max().unwrap());
}
