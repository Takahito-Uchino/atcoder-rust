use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        a: [Chars; n],
    }

    print!("{}", a[1][0]);
    for i in 0..n - 1 {
        print!("{}", a[0][i]);
    }
    println!();

    for i in 1..n - 1 {
        print!("{}", a[i + 1][0]);
        for j in 1..n - 1 {
            print!("{}", a[i][j]);
        }
        println!("{}", a[i - 1][n - 1]);
    }

    for i in 1..n {
        print!("{}", a[n - 1][i]);
    }
    println!("{}", a[n - 2][n - 1]);
}
