use proconio::input;

fn gcd(mut a : usize, mut b : usize) -> usize {
    while b != 0 {
        let t  : usize = b;
        b = a % b;
        a = t;
    }

    return a;
}

fn main() {
    input! {
        l : usize,
        r : usize,
    }

    let mut d : usize = r - l;
    while d > 0 {
        for i in l..=r - d {
            if gcd(i, i + d) == 1 {
                println!("{}", d);
                return;
            }
        }
        d -= 1;
    }
}

