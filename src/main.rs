use std::collections::HashMap;

use proconio::input;

fn main() {
    input! {
        n: usize,
        ac: [(usize, usize); n],
    }

    let mut mins = HashMap::new();

    for (a, c) in ac {
        mins.entry(c).or_insert(a);
        if mins[&c] > a {
            mins.insert(c, a);
        }
    }

    let mut ans = 0;
    for (_, v) in mins {
        if v > ans {
            ans = v;
        }
    }

    println!("{}", ans);
}
