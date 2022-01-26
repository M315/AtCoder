use proconio::input;

fn dfs(a : &Vec<Vec<usize>>, m : &mut usize, pairs : &Vec<usize>, n : usize) -> usize {
    // Check if all the pairs are set
    let mut zeros : Vec<usize> = Vec::new();
    for i in 0..pairs.len() {
        if pairs[i] == 0 {
            zeros.push(i);
        }
    }

    // Compute solution
    if zeros.len() == 1 {
        let mut sum : usize = 0;
        let mut used : Vec<bool> = vec![false; 2 * n];
        for i in 0..2 * n {
            if !used[i] {
                sum ^= a[i][pairs[i] - i - 1];
                used[i] = true;
                used[pairs[i]] = true;
            }
        }
        if sum > *m {
            *m = sum;
        }
    } else if zeros.len() == 2 * n {
        // Copy pairs
        let mut pairs_copy : Vec<usize> = pairs.to_vec();

        for i in 1..2 * n {
            pairs_copy[0] = i;

            let res : usize = dfs(a, m, &pairs_copy, n);

            if res > *m {
                *m = res;
            }
        }
    } else {
        let mut curr : usize = zeros[0];
        if curr == pairs[0] {
            curr = zeros[1];
        }

        for i in zeros {
            if curr != i && i != pairs[0] {
                // Copy pairs
                let mut pairs_copy : Vec<usize> = pairs.to_vec();

                pairs_copy[curr] = i;
                pairs_copy[i] = curr;

                let res : usize = dfs(a, m, &pairs_copy, n);

                if res > *m {
                    *m = res;
                }
            }
        }
    }

    return *m;
}

fn main() {
    input! {
        n : usize,
    }

    let mut a : Vec<Vec<usize>> = Vec::new();

    for i in 0..2 * n {
        input! {
            aux : [usize; 2 * n - i - 1],
        }
        a.push(aux);
    }

    let mut m : usize = 0;
    let mut pairs : Vec<usize> = vec![0; 2 * n];
    
    dfs(&a, &mut m, &mut pairs, n);

    println!("{}", m);
}
