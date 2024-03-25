use proconio::input;

fn main() {
    input! {
        n: usize,
    }

    let mut a = Vec::new();
    let mut b = Vec::new();
    let mut c = Vec::new();
    let mut d = Vec::new();

    for _ in 0..n {
        input! {
            an: usize,
            bn: usize,
            cn: usize,
            dn: usize,
        }
        a.push(an);
        b.push(bn);
        c.push(cn);
        d.push(dn);
    }

    let b_max = b.iter().max().unwrap();
    let d_max = d.iter().max().unwrap();

    let mut s = vec![vec![false; *d_max]; *b_max];

    for i in 0..n {
        for j in a[i]..b[i] {
            for k in c[i]..d[i] {
                s[j][k] = true;
            }
        }
    }

    let mut answer = 0;

    for s1 in s {
        for s2 in s1 {
            if s2 {
                answer += 1;
            }
        }
    }

    println!("{}", answer);
}
