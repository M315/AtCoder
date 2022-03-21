use proconio::input;

fn main() {
    input! {
        n : usize,
        l : usize,
        w : usize,
        a : [usize; n],
    }

    let mut traps : usize = 0;
    let mut covered : usize = 0;

    for p in a {
        if p > covered {
            traps += (p - covered + w - 1) / w;
        }
        covered = p + w;
    }

    if covered < l {
        traps += (l - covered + w - 1) / w;
    }

    println!("{}", traps);
}
