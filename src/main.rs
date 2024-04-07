use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n],
    }

    let (mut l, mut r) = (0, n - 1);

    while l <= r {
        let m = (l + r) / 2;
        if x < a[m] {
            r = m - 1;
        }
        if x == a[m] {
            println!("{}", m + 1);
            return;
        }
        if x > a[m] {
            l = m + 1;
        }
    }
}
