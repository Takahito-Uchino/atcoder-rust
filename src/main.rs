use proconio::input;

fn main() {
    input! {
        q: usize,
        queries: [(usize, usize); q],
    }

    let mut a = Vec::new();

    for query in queries {
        if query.0 == 1 {
            a.push(query.1);
        } else {
            println!("{}", a[a.len() - query.1]);
        }
    }
}
