use proconio::{input, marker::Chars};

fn main() {
    input! {
        n: usize,
        m: usize,
        s: [Chars; n],
    }

    let mut answer = Vec::new();

    for i in 0..n - 8 {
        for j in 0..m - 8 {
            let mut is_code = true;
            for k in 0..8 {
                for l in 0..8 {
                    if k < 3 && l < 3 && s[i + k][j + l] == '.' {
                        is_code = false;
                    } else if k > 5 && l > 5 && s[i + k][j + l] == '.' {
                        is_code = false;
                    } else if k == 3 && l < 4 && s[i + k][j + l] == '#' {
                        is_code = false;
                    } else if k == 5 && l > 4 && s[i + k][j + l] == '#' {
                        is_code = false;
                    } else if k < 4 && l == 3 && s[i + k][j + l] == '#' {
                        is_code = false;
                    } else if k > 4 && l == 5 && s[i + k][j + l] == '#' {
                        is_code = false;
                    }
                }
            }
            if is_code {
                answer.push((i + 1, j + 1));
            }
        }
    }

    for ans in answer {
        println!("{} {}", ans.0, ans.1);
    }
}
