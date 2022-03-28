use proconio::input;

fn main() {
    input! {
        n : usize,
        mut a : [usize; n],
    }

    a.sort();

    let mut win : bool = false;

    if a[a.len() - 2] + 1 < a[a.len() - 1] {
        win = true;
    } else if (a[a.len() - 1] - (a.len() - 1)) % 2 == 1 {
        win = true;
    }

    if win {
        println!("Alice");
    } else {
        println!("Bob");
    }
}
