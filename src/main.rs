use proconio::input;

fn main() {
    input! {
        n: usize,
        q: usize,
        events: [(usize, usize); q],
    }

    let mut counts = vec![0; n];

    for (i, x) in events {
        if i == 1 {
            counts[x - 1] += 1
        }
        if i == 2 {
            counts[x - 1] += 2
        }
        if i == 3 {
            println!("{}", if counts[x - 1] >= 2 { "Yes" } else { "No" })
        }
    }
}
