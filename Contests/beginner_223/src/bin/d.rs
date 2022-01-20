use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;

fn main() {
    input! {
        n : usize,
        m : usize,
    }

    let mut indeg : Vec<usize> = Vec::new();
    let mut out : Vec<Vec<usize>> = Vec::new();

    for _ in 0..n {
        indeg.push(0);
        out.push(Vec::new());
    }

    for _ in 0..m {
        input! {
            a : usize,
            b : usize,
        }
        
        indeg[b - 1] += 1;
        out[a - 1].push(b - 1);
    }

    let mut heap : BinaryHeap<Reverse<usize>> = BinaryHeap::new();

    for i in 0..n {
        if indeg[i] == 0 {
            heap.push(Reverse(i));
        }
    }

    let mut ans : Vec<usize> = Vec::new();

    while !heap.is_empty() {
        let Reverse(i) : Reverse<usize> = heap.pop().unwrap();

        ans.push(i);

        for &j in &out[i] {
            indeg[j] -= 1;
            if indeg[j] == 0 {
                heap.push(Reverse(j));
            }
        }
    }

    if ans.len() != n {
        println!("-1");
    } else {
        for k in ans {
            print!("{} ", k + 1);
        }
        println!();
    }
}
