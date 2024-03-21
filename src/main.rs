use proconio::input;

fn main() {
    input! {
        s: String,
    }

    let mut bx: Option<usize> = None;
    let mut by: Option<usize> = None;
    let mut k: Option<usize> = None;
    let mut rx: Option<usize> = None;
    let mut ry: Option<usize> = None;

    for (i, c) in s.chars().enumerate() {
        match c {
            'B' => {
                if bx.is_none() {
                    bx = Some(i + 1);
                } else if by.is_none() {
                    by = Some(i + 1);
                }
            }
            'K' => {
                if k.is_none() {
                    k = Some(i + 1);
                }
            }
            'R' => {
                if rx.is_none() {
                    rx = Some(i + 1);
                } else if ry.is_none() {
                    ry = Some(i + 1);
                }
            }
            _ => (),
        }
    }

    if let (Some(bx), Some(by), Some(rx), Some(k), Some(ry)) = (bx, by, rx, k, ry) {
        if ((bx % 2 == 0 && by % 2 == 1) || (bx % 2 == 1 && by % 2 == 0)) && (rx < k && k < ry) {
            println!("Yes");
        } else {
            println!("No");
        }
    } else {
        println!("No");
    }
}
