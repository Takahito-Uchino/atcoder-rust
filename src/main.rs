use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
    }

    let mut count = 0;

    for r in 1..=n {
        for b in 1..=n {
            if k <= n + r + b && k >= 1 + r + b {
                count += 1;
            }
        }
    }
    println!("{}", count);
}
