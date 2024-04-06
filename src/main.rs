use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
        q: usize,
        lr: [(usize, usize); q],
    }

    let mut counts = Vec::new();

    counts.push(if a[0] == 1 { 1 } else { 0 });

    for i in 1..n {
        counts.push(if a[i] == 1 {
            counts[i - 1] + 1
        } else {
            counts[i - 1]
        });
    }

    for (l, r) in lr {
        let is_true = if l == 1 {
            counts[r - 1]
        } else {
            counts[r - 1] - counts[l - 2]
        };
        let is_false = r - l + 1 - is_true;
        if is_true > is_false {
            println!("win");
        } else if is_true < is_false {
            println!("lose");
        } else {
            println!("draw");
        }
    }
}
