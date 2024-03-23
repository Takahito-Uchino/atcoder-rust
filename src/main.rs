use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    for i in 0..n - 1 {
        print!("{} ", a[i] * a[i + 1]);
    }

    println!();
}
