use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    for i in a {
        if i % k == 0 {
            print!("{} ", i / k);
        }
    }

    println!()
}
