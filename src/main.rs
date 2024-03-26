use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: Chars,
        t: Chars,
    }

    let mut is_prefix = true;
    let mut is_postfix = true;

    for i in 0..n {
        if s[i] != t[i] {
            is_prefix = false;
        }
        if s[n - 1 - i] != t[m - 1 - i] {
            is_postfix = false;
        }
    }

    if is_prefix && is_postfix {
        println!("0");
    } else if is_prefix {
        println!("1");
    } else if is_postfix {
        println!("2");
    } else {
        println!("3");
    }
}
