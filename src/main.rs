use proconio::input;

fn main() {
    input! {
        h: usize,
        w: usize,
        n: usize,
    }

    let mut grid = vec![vec![false; w]; h];

    let dx: Vec<isize> = vec![-1, 0, 1, 0];
    let dy: Vec<isize> = vec![0, 1, 0, -1];

    let (mut x, mut y) = (0isize, 0isize);
    let mut m: isize = 0;

    for _ in 0..n {
        let ux = x.rem_euclid(h as isize) as usize;
        let uy = y.rem_euclid(w as isize) as usize;

        if grid[ux][uy] == false {
            grid[ux][uy] = true;
            m += 1;
        } else {
            grid[ux][uy] = false;
            m += 3;
        }
        m %= 4;
        x += dx[m as usize];
        y += dy[m as usize];
    }

    for i in 0..h {
        for j in 0..w {
            print!("{}", if grid[i][j] { '#' } else { '.' })
        }
        println!();
    }
}
