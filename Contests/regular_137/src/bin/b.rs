use proconio::input;

fn main() {
    input! {
        n : usize,
        a : [i32; n],
    }

    let mut zeros : i32 = 0;
    let mut ones : i32 = 0;
    let mut min : i32 = 0;
    let mut max : i32 = 0;
    let mut curr : i32 = 0;

    for i in 0..n {
        if a[i] == 0 {
            curr -= 1;
        } else {
            curr += 1;
        }

        zeros = zeros.min(curr - max);
        ones = ones.max(curr - min);
        min = min.min(curr);
        max = max.max(curr);
    }
   
    println!("{}", ones - zeros + 1);
}
