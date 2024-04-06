use proconio::input;

fn main() {
    input! {
        d: usize,
        n: usize,
        lr: [(usize, usize); n],
    }

    let mut attendances = vec![0; d];

    for (l, r) in lr {
        attendances[l - 1] += 1;
        if r < d {
            attendances[r] -= 1;
        }
    }

    let mut answer = vec![0; d];

    answer[0] = attendances[0];
    for i in 1..d {
        answer[i] = answer[i - 1] + attendances[i];
    }

    for ans in answer {
        println!("{}", ans);
    }
}
