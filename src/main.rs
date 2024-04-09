use proconio::input;

fn enumerate(a: Vec<i64>) -> Vec<i64> {
    let mut sum_list = Vec::new();
    for i in 0..(1 << a.len()) {
        let mut sum = 0;
        for j in 0..a.len() {
            if (i >> j) & 1 == 1 {
                sum += a[j];
            }
        }
        sum_list.push(sum);
    }
    sum_list
}

fn main() {
    input! {
        n: usize,
        k: i64,
        a: [i64; n],
    }

    let (l1, l2) = a.split_at(n / 2);
    let sum1 = enumerate(l1.to_vec());
    let sum2 = enumerate(l2.to_vec());

    let mut sum1 = sum1;
    let mut sum2 = sum2;
    sum1.sort();
    sum2.sort();

    for &s in &sum1 {
        if let Ok(_) = sum2.binary_search(&(k - s)) {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

