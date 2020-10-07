use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        (Usize1) => (read!(usize) - 1);
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (n, m) = read!((usize, usize));
    let mut gs = vec![vec![std::u64::MAX / 2; n]; n];
    for i in 0..n {
        gs[i][i] = 0;
    }
    for (a, b, t) in read!([(Usize1, Usize1, u64); m]) {
        gs[a][b] = t;
        gs[b][a] = t;
    }
    for k in 0..n {
        for i in 0..n {
            for j in 0..n {
                gs[i][j] = gs[i][j].min(gs[i][k] + gs[k][j]);
            }
        }
    }

    let d = gs.iter().map(|vs| vs.iter().max().unwrap()).min().unwrap();
    println!("{}", d);
}
