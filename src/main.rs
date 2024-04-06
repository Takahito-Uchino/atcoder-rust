use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        x: [[usize; w]; h],
        q: usize,
        abcd: [(usize, usize, usize, usize); q],
    }

    let mut z = vec![vec![0; w + 1]; h + 1];

    for i in 0..h {
        for j in 0..w {
            z[i + 1][j + 1] = z[i + 1][j] + x[i][j];
        }
    }

    for j in 0..w {
        for i in 0..h {
            z[i + 1][j + 1] += z[i][j + 1];
        }
    }

    for (a, b, c, d) in abcd {
        println!("{}", z[c][d] + z[a - 1][b - 1] - z[a - 1][d] - z[c][b - 1]);
    }
}
