use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
        abcd: [(usize, usize, usize, usize); n],
    }

    let mut s = vec![vec![0; w + 2]; h + 2];

    for (a, b, c, d) in abcd {
        s[a][b] += 1;
        s[c + 1][d + 1] += 1;
        s[c + 1][b] -= 1;
        s[a][d + 1] -= 1;
    }

    let mut t = vec![vec![0; w + 2]; h + 2];

    for i in 1..=h {
        for j in 1..=w {
            t[i][j] = t[i][j - 1] + s[i][j];
        }
    }

    for j in 1..=w {
        for i in 1..=h {
            t[i][j] += t[i - 1][j];
        }
    }

    for i in 1..=h {
        for j in 1..=w {
            print!("{} ", t[i][j]);
        }
        println!()
    }
}
