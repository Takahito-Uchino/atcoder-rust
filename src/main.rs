use proconio::input;

fn main() {
    input! {
        n: usize,
        m: usize,
        ab: [(usize, usize); m],
    }

    let mut strongest = vec![true; n];

    for (_, b) in ab {
        strongest[b - 1] = false;
    }

    let mut count = 0;
    let mut nums = Vec::new();

    for i in 0..n {
        if strongest[i] {
            count += 1;
            nums.push(i + 1);
        }
    }

    if count == 1 {
        println!("{}", nums[0]);
    } else {
        println!("-1");
    }
}
