use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        d: usize,
        s: [Chars; n],
    }

    let mut vacations = vec![false; d];

    for i in 0..d {
        let mut is_vacation = true;
        for j in 0..n {
            if s[j][i] == 'x' {
                is_vacation = false;
            }
        }
        if is_vacation {
            vacations[i] = true;
        }
    }

    let mut answer = Vec::new();
    let mut current = 0;
    for i in 0..d {
        if vacations[i] {
            current += 1;
        } else {
            answer.push(current);
            current = 0;
        }
        if i == d - 1 {
            answer.push(current);
        }
    }

    println!("{}", answer.iter().max().unwrap());
}
