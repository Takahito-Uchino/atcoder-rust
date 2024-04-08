use proconio::input;

fn main() {
    input! {
        n: f64,
    }

    let (mut l, mut r) = (0., 100.);

    for _ in 0..20 {
        let m = (l + r) / 2.;
        if m * m * m + m > n {
            r = m;
        } else {
            l = m;
        }
    }

    println!("{}", l);
}
