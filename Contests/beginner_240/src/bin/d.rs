use proconio::input;

fn main() {
    input! {
        n : usize,
        a : [usize; n],
    }

    let mut v : Vec<(usize,usize)> = Vec::new();
    let mut l : usize = 0;

    for x in a {
        if v.is_empty() {
            v.push((x, 1));
            l = 1;
        } else {
            if x != v[v.len() - 1].0 {
                v.push((x, 1));
            } else {
                let len : usize = v.len();
                v[len - 1].1 += 1;
            }
            l += 1;
        }

        if v[v.len() - 1].0 == v[v.len() - 1].1 {
            l -= v[v.len() - 1].0;
            v.pop();
        }

        println!("{}", l);
    }
}
