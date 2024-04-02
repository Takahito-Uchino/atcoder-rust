use proconio::{input, marker::Chars};

fn main() {
    input! {
        s: Chars,
    }

    let (mut count_a, mut count_b, mut count_c) = (0, 0, 0);

    for c in &s {
        match c {
            'A' => count_a += 1,
            'B' => count_b += 1,
            'C' => count_c += 1,
            _ => unreachable!(),
        }
    }

    let mut answer = Vec::new();
    for _ in 0..count_a {
        answer.push('A');
    }
    for _ in 0..count_b {
        answer.push('B');
    }
    for _ in 0..count_c {
        answer.push('C');
    }

    if &answer == &s {
        println!("Yes");
    } else {
        println!("No");
    }
}
