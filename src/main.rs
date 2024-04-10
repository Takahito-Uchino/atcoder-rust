use proconio::input;

fn main() {
    input! {
        n: usize,
        h: [isize; n],
    }

    let mut dp = Vec::new();
    dp.push(0);
    dp.push((h[1] - h[0]).abs());

    for i in 2..n {
        dp.push((dp[i - 1] + (h[i - 1] - h[i]).abs()).min(dp[i - 2] + (h[i - 2] - h[i]).abs()));
    }

    println!("{}", dp[n - 1]);
}

