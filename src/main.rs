use proconio::input;
use std::cmp::max;

fn main() {
    input! {
        n: usize,
        t: i32,
        c: [i32; n],
        r: [i32; n],
    }

    let mut tmax = (-1, -1);
    let mut lmax = (-1, -1);

    for i in 0..n {
        if c[i] == t {
            tmax = max(tmax, (r[i], (i + 1) as i32));
        }
        if c[i] == c[0] {
            lmax = max(lmax, (r[i], (i + 1) as i32));
        }
    }

    if tmax.0 != -1 {
        println!("{}", tmax.1);
    } else {
        println!("{}", lmax.1);
    }
}
