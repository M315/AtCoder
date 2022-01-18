use proconio::input;
use std::collections::HashMap;

fn main() {
    input! {
        n : usize,
        q : usize,
    }

    let mut nums : HashMap<usize, Vec<usize>> = HashMap::new();

    for i in 1..=n {
        input! {
            a : usize,
        }

        if nums.contains_key(&a) {
            nums.get_mut(&a).unwrap().push(i);
        } else {
            nums.insert(a, vec![i]);
        }
    }

    for _ in 0..q {
        input! {
            x : usize,
            k : usize,
        }

        if nums.contains_key(&x) {
            if nums[&x].len() >= k {
                println!("{}", nums[&x][k - 1]);
            } else {
                println!("-1");
            }
        } else {
            println!("-1");
        }
    }
}
