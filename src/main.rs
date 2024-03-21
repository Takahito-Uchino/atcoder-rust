use proconio::input;
fn main() {
    input! {
        r: usize,
        c: usize,
        b: [String; r],
    }

    let mut blasted = vec![vec![false; c]; r];

    for i in 0..r {
        for j in 0..c {
            if let Some(power) = b[i].chars().nth(j).unwrap().to_digit(10) {
                for ni in 0..r {
                    for nj in 0..c {
                        if (max(i, ni) - min(i, ni)) + (max(j, nj) - min(j, nj)) <= power as usize {
                            blasted[ni][nj] = true
                        }
                    }
                }
            }
        }
    }

    for i in 0..r {
        for j in 0..c {
            if blasted[i][j] {
                print!(".");
            } else {
                print!("{}", b[i].chars().nth(j).unwrap())
            }
        }
        println!();
    }
}

fn min(a: usize, b: usize) -> usize {
    if a < b {
        a
    } else {
        b
    }
}

fn max(a: usize, b: usize) -> usize {
    if a > b {
        a
    } else {
        b
    }
}
