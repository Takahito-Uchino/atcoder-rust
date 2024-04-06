use proconio::input;

fn main() {
    input! {
        t: usize,
        n: usize,
        lr:[(usize, usize); n],
    }

    let mut rise_or_fall = vec![0; t];

    for (l, r) in lr {
        rise_or_fall[l] += 1;
        if r < t {
            rise_or_fall[r] -= 1;
        }
    }

    let mut answers = vec![0; t];

    answers[0] = rise_or_fall[0];
    for i in 1..t {
        answers[i] = answers[i - 1] + rise_or_fall[i];
    }

    for ans in answers {
        println!("{}", ans);
    }
}
