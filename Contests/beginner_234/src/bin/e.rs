use proconio::input;

fn digit_to_num(d : &Vec<u8>) -> u64 {
    let mut ret : u64 = 0;
    let mut m : u64 = 1;
    let n : usize = d.len();

    for i in 0..n { 
        ret += d[n - 1 - i] as u64 * m;
        m *= 10;
    }

    return ret;
}

fn next_arithmetic(o : &mut Vec<u8>) -> Vec<u8> {
    let n : usize = o.len();
    let mut next : Vec<u8> = vec![0; n];
    for i in 0..n {
        next[i] = o[i];
    }

    if n <= 2 {
        return next;
    }

    let diff_max : i32 = ((10 + n - 1) / n) as i32;

    loop {
        let mut found : bool = true;

        for diff in -diff_max..=diff_max {

            found = true;

            for i in 0..n - 1 {
                if next[i] as i32 + diff >= 0 && next[i] as i32 + diff <= 9 {
                    next[i + 1] = (next[i] as i32 + diff) as u8;
                }
                else { 
                    found = false;
                    break;
                }
            }

            if found {
                if digit_to_num(&next) >= digit_to_num(&o)
                {
                    break;
                }
                else
                {
                    found = false;
                }
            }
        }

        if found {
            break;
        }

        next[0] += 1;
        assert!(next[0] < 10);
    }

    return next;
}

fn main() {
    input! {
        mut x : u64,
    }

    // Get the digits of x
    let mut d : Vec<u8> = Vec::new();

    while x > 0 {
        d.push((x % 10) as u8);
        x /= 10;
    }
    d.reverse();

    d = next_arithmetic(&mut d);

    // Transform digits to number
    println!("{}", digit_to_num(&d));
}
