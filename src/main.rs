use proconio::input;

fn main() {
    input! {
        n: usize,
        x: usize,
        a: [usize; n - 1],
    }

    let total = a.iter().sum::<usize>();
    let max = a.iter().max().unwrap();
    let min = a.iter().min().unwrap();

    if x > total - min {
        println!("-1");
        return;
    }

    for i in 0..=100 {
        if i < *min && x <= total - max {
            println!("{}", i);
            return;
        } else if i <= *max && x <= total + i - max - min {
            println!("{}", i);
            return;
        } else if i > *max && x <= total - min {
            println!("{}", i);
            return;
        }
    }
}
