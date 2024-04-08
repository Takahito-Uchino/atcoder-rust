use proconio::input;

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let mut s = vec![0i64; n + 1];
    let mut r = vec![0usize; n + 1];

    for i in 1..=n {
        s[i] = s[i - 1] + a[i - 1];
    }

    for i in 1..=n {
        if i == 1 {
            r[i] = 0;
        } else {
            r[i] = r[i - 1];
        }
        while r[i] < n && sum(&s, i, r[i] + 1) <= k {
            r[i] += 1;
        }
    }

    let mut answer = 0i64;
    for i in 1..=n {
        answer += (r[i] as i64) - (i as i64) + 1;
    }

    println!("{}", answer);
}

fn sum(s: &Vec<i64>, l: usize, r: usize) -> i64 {
    s[r] - s[l - 1]
}
