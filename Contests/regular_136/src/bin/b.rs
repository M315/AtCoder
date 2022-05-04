use proconio::input;

fn main() {
    input! {
        n : usize,
        mut a : [usize; n],
        mut b : [usize; n],
    }

    let mut asort = a.clone();
    let mut bsort = b.clone();

    if n > 2 {
        asort.sort();
        bsort.sort();
    }

    for i in 0..n {
        if asort[i] != bsort[i] {
            println!("No");
            return;
        }
    }

    let mut inv = (0, 0);
    for i in 0..n {
        for j in i + 1..n {
            if a[i] > a[j] { inv.0 ^= 1; }
            if b[i] > b[j] { inv.1 ^= 1; }
            if a[i] == a[j] {
                println!("Yes");
                return;
            }
        }
    }

    if inv.0 == inv.1 {
        println!("Yes");
    } else {
        println!("No");
    }
}
