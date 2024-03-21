use proconio::input;

fn main() {
    input! {
        n: usize,
        mut a: [[usize; n]; n],
        b: [[usize; n]; n],
    }

    for _ in 0..4 {
        let mut ok = true;
        for i in 0..n {
            for j in 0..n {
                if a[i][j] == 1 && b[i][j] == 0 {
                    ok = false;
                }
            }
        }
        if ok {
            println!("Yes");
            return;
        }
        a = rotate(a, n);
    }

    println!("No");
}

fn rotate(a: Vec<Vec<usize>>, n: usize) -> Vec<Vec<usize>> {
    let mut res = vec![vec![0; n]; n];

    for i in 0..n {
        for j in 0..n {
            res[j][n - 1 - i] = a[i][j]
        }
    }

    res
}
