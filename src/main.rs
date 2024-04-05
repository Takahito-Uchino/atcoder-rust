use proconio::input;

fn main() {
    input! {
        n: usize,
    }
    for i in (0..10).rev() {
        print!("{}", n / 2usize.pow(i) % 2);
    }
    println!()
}
