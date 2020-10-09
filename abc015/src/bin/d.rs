use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (w, n, m) = read!((usize, usize, usize));
    let abs = read!([(usize, usize); n]);
    let mut dp = vec![vec![vec![0; n]; m + 1]; w + 1];
    for i in 0..=w {
        for j in 1..=m {
            for k in 0..n {
                dp[i][j][k] = if i >= abs[k].0 {
                    if k > 0 {
                        dp[i][j][k - 1].max(dp[i - abs[k].0][j - 1][k - 1] + abs[k].1)
                    } else {
                        abs[k].1
                    }
                } else if k > 0 {
                    dp[i][j][k - 1]
                } else {
                    0
                };
            }
        }
    }
    println!("{}", dp[w][m][n - 1]);
}
