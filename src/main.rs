use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let mut answer = vec![false; n];

    for i in 0..n {
        if !answer[i] {
            answer[a[i] - 1] = true
        }
    }

    let mut count = 0;

    for ans in &answer {
        if !ans {
            count += 1
        }
    }

    println!("{}", count);

    for i in 0..n {
        if !answer[i] {
            print!("{} ", i + 1)
        }
    }
    println!("")
}
