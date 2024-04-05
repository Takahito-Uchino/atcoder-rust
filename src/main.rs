use proconio::input;

fn main() {
    input! {
        n: String,
    }

    let length = n.len();
    let mut answer = 0;

    for (i, c) in n.chars().enumerate() {
        if c == '1' {
            answer += 2usize.pow((length - 1 - i) as u32);
        }
    }

    println!("{}", answer);
}
