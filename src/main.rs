use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n - 1],
        b: [usize; n - 2],
    }

    let mut dp = Vec::new();
    dp.push(0);
    dp.push(a[0]);

    for i in 2..n {
        dp.push((dp[i - 1] + a[i - 1]).min(dp[i - 2] + b[i - 2]));
    }

    println!("{}", dp[n - 1]);
}

