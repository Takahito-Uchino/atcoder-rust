use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
    }

    let mut r = vec![0; 100009];
    for i in 1..n {
        if i == 1 {
            r[i] = 1;
        } else {
            r[i] = r[i - 1];
        }
        while r[i] < n && a[r[i]] - a[i - 1] <= k {
            r[i] += 1;
        }
    }

    let mut answer = 0;
    for i in 1..n {
        answer += r[i] - i;
    }

    println!("{}", answer);
}
