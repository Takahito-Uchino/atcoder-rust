use proconio::input;

fn main() {
    input! {
        n: usize,
        k: usize,
        a: [usize; n],
        b: [usize; n],
        c: [usize; n],
        d: [usize; n],
    }

    let mut p = Vec::new();
    let mut q = Vec::new();

    for i in 0..n {
        for j in 0..n {
            p.push(a[i] + b[j]);
        }
    }

    for i in 0..n {
        for j in 0..n {
            q.push(c[i] + d[j]);
        }
    }

    q.sort();

    for i in 0..n*n {
        if q.binary_search(&(k - p[i])).is_ok() {
            println!("Yes");
            return;
        }
    }

    println!("No");
}

