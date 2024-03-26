use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        s: [Chars; n],
    }

    let mut wins = vec![0; n];

    for i in 0..n {
        for j in 0..n {
            if s[i][j] == 'o' {
                wins[i] += 1;
            }
        }
    }

    let mut bucket = vec![Vec::new(); n];

    for i in 0..n {
        bucket[wins[i]].push(i);
    }

    let mut answer = Vec::new();

    for i in (0..n).rev() {
        for &j in &bucket[i] {
            answer.push(j + 1);
        }
    }

    println!(
        "{}",
        answer
            .iter()
            .map(|x| x.to_string())
            .collect::<Vec<String>>()
            .join(" ")
    );
}
