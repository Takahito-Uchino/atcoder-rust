use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        p: [usize; n],
        q: [usize; n],
    }

    for i in &p {
        for j in &q {
            if i + j == k {
                println!("Yes");
                return;
            }
        }
    }
    println!("No");
}
