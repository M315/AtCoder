use proconio::input;

fn main() {
    input! {
        t : u64,
    }

    for _ in 0..t {
        input! {
            a : u64,
            s : u64,
        }

        if 2 * a <= s && (s - 2 * a) & a == 0 {
            println!("Yes");
        } else {
            println!("No");
        }
    }
}

