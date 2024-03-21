use proconio::input;
fn main() {
    input! {
        s: [String; 8],
    }

    for i in 0..8 {
        for j in 0..8 {
            if s[i].chars().nth(j).unwrap() == '*' {
                println!("{}{}", (j as u8 + 97) as char, 8 - i);
            }
        }
    }
}
