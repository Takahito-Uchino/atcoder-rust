use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut l = 0;
    let mut r = 1000000000;

    while l < r {
        let m = (l + r) / 2;
        if check(m, n, k, a.clone()) {
            r = m;
        } else {
            l = m + 1;
        }
    }

    println!("{}", l);
}

fn check(x: usize, n: usize, k: usize, a: Vec<usize>) -> bool {
    let mut sum = 0;
    for i in 0..n {
        sum += x / a[i];
    }
    sum >= k
}
