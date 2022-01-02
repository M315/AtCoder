use::proconio::input;
use::proconio::marker::Usize1;

/// Struct from crate contest_algorithms

/// Represents a union of disjoint sets. Each set's elements are arranged in a
/// tree, whose root is the set's representative.
pub struct DisjointSets {
    parent: Vec<usize>,
}

impl DisjointSets {
    /// Initializes disjoint sets containing one element each.
    pub fn new(size: usize) -> Self {
        Self {
            parent: (0..size).collect(),
        }
    }

    /// Finds the set's representative. Do path compression along the way to make
    /// future queries faster.
    pub fn find(&mut self, u: usize) -> usize {
        let pu = self.parent[u];
        if pu != u {
            self.parent[u] = self.find(pu);
        }
        self.parent[u]
    }

    /// Merges the sets containing u and v into a single set containing their
    /// union. Returns true if u and v were previously in different sets.
    pub fn merge(&mut self, u: usize, v: usize) -> bool {
        let (pu, pv) = (self.find(u), self.find(v));
        self.parent[pu] = pv;
        pu != pv
    }
}

fn main()
{
    input!
    {
        n : usize,
        m : usize,
    }

    let mut d : Vec<usize> = vec![0; n];

    let mut ds = DisjointSets::new(n);
    
    for _ in 0..m
    {
        input!
        {
            a : Usize1,
            b : Usize1,
        }

        if !ds.merge(a, b)
        {
            println!("No");
            return;
        }

        d[a] += 1;
        d[b] += 1;
    }

    for i in 0..n
    {
        if d[i] > 2
        {
            println!("No");
            return;
        }
    }

    println!("Yes");
}
