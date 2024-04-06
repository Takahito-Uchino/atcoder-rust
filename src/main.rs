use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        a: [usize; n],
        lr: [(usize, usize); q],
    }

    let mut totals = Vec::new();

    totals.push(a[0]);

    for i in 1..n {
        totals.push(totals[i - 1] + a[i]);
    }

    for (l, r) in lr {
        if l == 1 {
            println!("{}", totals[r - 1]);
        } else {
            println!("{}", totals[r - 1] - totals[l - 2]);
        }
    }
}
