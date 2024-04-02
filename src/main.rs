use proconio::input;

fn main() {
    input! {
        mut n: usize,
    }

    while n % 2 == 0 {
        n /= 2;
    }

    while n % 3 == 0 {
        n /= 3;
    }

    if n == 1 {
        println!("Yes")
    } else {
        println!("No")
    }
}
