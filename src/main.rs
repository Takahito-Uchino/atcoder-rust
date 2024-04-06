use proconio::input;

fn main() {
    input! {
        n: usize,
        xy: [(usize, usize); n],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }

    let mut z = vec![vec![0; 1509]; 1509];

    for (x, y) in xy {
        z[x][y] += 1;
    }

    let mut t = vec![vec![0; 1509]; 1509];
    for i in 1..1509 {
        for j in 1..1509 {
            t[i][j] = t[i][j - 1] + z[i][j];
        }
    }
    for j in 1..1509 {
        for i in 1..1509 {
            t[i][j] += t[i - 1][j];
        }
    }

    for (a, b, c, d) in abcd {
        println!("{}", t[c][d] + t[a - 1][b - 1] - t[a - 1][d] - t[c][b - 1]);
    }
}
