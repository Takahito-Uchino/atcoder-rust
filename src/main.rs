use proconio::{input, marker::Chars};

fn main() {
    input! {
        h: isize,
        w: isize,
        a: [Chars; h],
        b: [Chars; h],
    }

    for s in 0..h {
        for t in 0..w {
            let mut ok = true;
            for i in 0..h {
                for j in 0..w {
                    if a[((i - s + h) % h) as usize][((j - t + w) % w) as usize]
                        != b[i as usize][j as usize]
                    {
                        ok = false;
                    }
                }
            }
            if ok {
                println!("Yes");
                return;
            }
        }
    }

    println!("No");
}
