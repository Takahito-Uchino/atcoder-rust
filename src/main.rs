use proconio::input;

fn main() {
    input! {
        n: usize,
        mut x: [usize; 5 * n],
    }

    for _ in 0..n {
        let max = x.iter().max().unwrap();
        let max_index = x.iter().position(|&v| v == *max).unwrap();
        x.remove(max_index);
    }

    for _ in 0..n {
        let min = x.iter().min().unwrap();
        let min_index = x.iter().position(|&v| v == *min).unwrap();
        x.remove(min_index);
    }

    println!("{}", x.iter().sum::<usize>() as f64 / (3 * n) as f64)
}
