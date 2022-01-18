use proconio::input;

fn main() {
    input! {
        mut k : u64,
    }

    let mut ret : Vec<u64> = Vec::new();

    while k > 0 {
        ret.push(k % 2);
        k /= 2;
    }

    ret.reverse();

    for d in ret {
        print!("{}", d * 2);
    }
    println!();
}

