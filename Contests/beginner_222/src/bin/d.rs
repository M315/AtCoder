use proconio::input;

static M : usize = 998_244_353;

fn main() {
    input! {
        n : usize,
        a : [usize; n],
        b : [usize; n],
    }

    let mut dp : Vec<Vec<usize>> = vec![vec![1; b[n - 1] + 1]; n];
    
    for j in (0..b[n - 1]).rev() {
        dp[n - 1][j] = dp[n - 1][j + 1];

        if j >= a[n - 1] {
            dp[n - 1][j] += 1;
        }
    }

    for i in (0..n - 1).rev() {
        dp[i][b[i]] = dp[i + 1][b[i]];
        
        dp[i][b[i]] %= M;

        for j in (0..b[i]).rev() {
            if j >= a[i] {
                dp[i][j] = dp[i][j + 1] + dp[i + 1][j];
            } else {
                dp[i][j] = dp[i][j + 1];
            }

            dp[i][j] %= M;
        }
    }

    println!("{}", dp[0][a[0]]);
}
