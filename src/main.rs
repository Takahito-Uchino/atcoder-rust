use proconio::input;

fn main() {
    input! {
        w: usize,
        b: usize,
    }

    let t = "wbwbwwbwbwbw".chars().collect::<Vec<_>>();

    for i in 0..12 {
        let (mut nw, mut nb) = (0, 0);
        for j in 0..(w + b) {
            if t[(i + j) % 12] == 'w' {
                nw += 1;
            } else {
                nb += 1;
            }
        }
        if w == nw && b == nb {
            println!("Yes");
            return;
        }
    }

    println!("No");
}
