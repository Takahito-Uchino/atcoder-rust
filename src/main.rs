use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        d: usize,
        lr: [(usize, usize); d],
    }

    let mut p = vec![0; n + 1];

    p[1] = a[0];
    for i in 2..=n {
        p[i] = p[i - 1].max(a[i - 1]);
    }

    let mut q = vec![0; n + 1];

    q[n] = a[n - 1];
    for i in (1..n).rev() {
        q[i] = q[i + 1].max(a[i - 1]);
    }

    for (l, r) in lr {
        println!("{}", p[l - 1].max(q[r + 1]));
    }
}
