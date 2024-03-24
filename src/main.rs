use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = Vec::new();

    for _ in 0..n {
        input! {
            c: usize,
            vets: [usize; c],
        }
        a.push(vets);
    }

    input! {
        x: usize,
    }

    let mut is_vet = Vec::new();

    for i in 0..n {
        if a[i].contains(&x) {
            is_vet.push(i);
        }
    }

    let mut cmin = 37;
    for i in &is_vet {
        cmin = cmin.min(a[*i].len());
    }

    let mut answer = Vec::new();
    for i in &is_vet {
        if a[*i].len() == cmin {
            answer.push(i + 1);
        }
    }
    println!("{}", answer.len());
    for ans in answer {
        print!("{} ", ans);
    }
}
