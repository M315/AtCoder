use proconio::input;

fn main() {
    input! {
        n : usize,
    }

    let mut x : Vec<(i32, i32)> = Vec::new();

    for _ in 0..n {
        input! {
            a : i32,
            b : i32,
        }

        x.push((a, 1));
        x.push((a + b, -1));
    }

    x.sort();

    let mut ans : Vec<i32> = vec![0; n + 1];
    let mut count : i32 = 0;

    for i in 0..x.len() - 1 {
        count += x[i].1;
        ans[count as usize] += x[i + 1].0 - x[i].0;
    }

    for i in 1..ans.len() {
        print!("{} ", ans[i]);
    }
    println!();
}
