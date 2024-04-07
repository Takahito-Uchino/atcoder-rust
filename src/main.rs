use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [usize; n],
        q: usize,
        x: [usize; q],
    }

    a.sort();

    for i in x {
        let result = a.binary_search(&i);
        match result {
            Ok(index) => println!("{}", index),
            Err(index) => println!("{}", index),
        }
    }
}
