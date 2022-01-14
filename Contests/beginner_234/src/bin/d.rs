use proconio::input;
use std::collections::BinaryHeap;
use std::cmp::Reverse;
 
fn main() {
    input! {
        n : usize,
        mut k : usize,
        p : [usize; n],
    }
 
    let mut s : BinaryHeap<Reverse<usize>> = BinaryHeap::new();
 
    for i in 0..n {
        s.push(Reverse(p[i]));

        while s.len() > k
        {
            s.pop();
        }
        
        if s.len() == k {
            let &Reverse(head) : &Reverse<usize> = s.peek().unwrap(); 
            println!("{}", head);
        }
    }
}
