use::proconio::input;
use::std::cmp;

fn main()
{
    input!
    {
        n : i64,
        a : i64,
        b : i64,
        p : i64,
        q : i64,
        r : i64,
        s : i64,
    }

    // Compute the constant for the ranges in the diagonals that are painted black
    let a1 : i64 = cmp::max(1 - a, 1 - b);
    let b1 : i64 = cmp::max(n - a, n - b);

    let a2 : i64 = cmp::max(1 - a, b - n);
    let b2 : i64 = cmp::max(n - a, b - 1);

    for i in p..=q
    {
        for j in r..=s
        {
            // Check weather (i,j) is in the first diagonal
            // This is (i, j) = (A + k, B + k) and k in the diagonal range
            let k : i64 = i - a;
            if k == j - b && k >= a1 && k <= b1
            {
                print!("#");
                continue;
            }

            // Check weather (i,j) is in the second diagonal
            // This is (i, j) = (A + k, B - k) and k in the diagonal range
            if k == b - j && k >= a2 && k <= b2
            {
                print!("#");
                continue;
            }
            
            print!(".");
        }
        println!();
    }
}
