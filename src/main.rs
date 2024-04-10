use std::collections::HashSet;

use proconio::input;

fn main() {
    input! {
        n: usize,
        a: [usize; n],
    }

    let a_set = a.iter().collect::<HashSet<_>>();
    let mut t = a_set.iter().map(|&&x| x).collect::<Vec<_>>();
    t.sort_unstable();

    let mut b = Vec::new();
    for i in a {
        match t.binary_search(&i) {
            Ok(index) => b.push(index + 1),
            Err(_) => unreachable!(),
        }
    }

    println!("{}", b.into_iter().map(|n| n.to_string()).collect::<Vec<_>>().join(" "));
}

