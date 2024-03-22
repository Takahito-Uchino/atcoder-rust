use proconio::input;

fn main() {
    input! {
        n: usize,
        _: usize,
    }

    let mut p = Vec::new();
    let mut c = Vec::new();
    let mut f = Vec::new();

    for _ in 0..n {
        input! {
            price: usize,
            count: usize,
            functions: [usize; count],
        }

        p.push(price);
        c.push(count);
        f.push(functions);
    }

    for i in 0..n {
        for j in 0..n {
            if i != j {
                let mut ok = true;
                if p[i] < p[j] {
                    ok = false;
                }
                for v in &f[i] {
                    if !f[j].contains(v) {
                        ok = false;
                    }
                }
                if ok && p[i] == p[j] && f[i].len() >= f[j].len() {
                    ok = false;
                }
                if ok {
                    println!("Yes");
                    return;
                }
            }
        }
    }
    println!("No");
}
