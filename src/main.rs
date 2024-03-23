use std::{collections::HashSet, iter::FromIterator};

use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut total = 0;

    let a_set: HashSet<usize> = HashSet::from_iter(a.iter().cloned());

    for i in a_set {
        if k >= i {
            total += i;
        }
    }

    println!("{}", (1 + k) * k / 2 - total);
}
