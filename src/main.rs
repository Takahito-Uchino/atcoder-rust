use proconio::input;

fn main() {
    input! {
        m: usize,
        d: [usize; m],
    }

    let mid = (d.iter().sum::<usize>() + 1) / 2;

    let mut current = 0;

    for i in 0..m {
        if mid > current && mid <= current + d[i] {
            println!("{} {}", i + 1, mid - current);
            return;
        } else {
            current += d[i];
        }
    }
}
