use proconio::input;

fn main() {
    input! {
        n : usize,
        q : usize,
    }

    let mut parent : Vec<usize> = (0..=n).collect();
    let mut child : Vec<usize> = (0..=n).collect();

    for _ in 0..q {
        input! {
            t : usize,
        }

        if t == 1 {
            input! {
                x : usize,
                y : usize,
            }

            child[x] = y;
            parent[y] = x;
        }

        if t == 2 {
            input! {
                x : usize,
                y : usize,
            }

            child[x] = x;
            parent[y] = y;
        }

        if t == 3 {
            input! {
                mut x : usize,
            }

            while child[x] != x {
                x = child[x];
            }

            let mut len : usize = 1;

            // Find frot car
            while parent[x] != x {
                x = parent[x];
                len += 1;
            }

            // Print the cars
            print!("{} ", len);
            while child[x] != x {
                print!("{} ", x);
                x = child[x];
            }

            println!("{}", x);
        }
    }
}
