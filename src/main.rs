use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut answer = String::new();

    let mut divisors = Vec::new();

    for i in 1..=9 {
        if n % i == 0 {
            divisors.push(i);
        }
    }

    for i in 0..=n {
        let mut jj = false;
        for j in &divisors {
            if i % (n / j) == 0 {
                answer.push_str(&j.to_string());
                jj = true;
                break;
            }
        }
        if !jj {
            answer.push_str("-");
        }
    }

    println!("{}", answer);
}
