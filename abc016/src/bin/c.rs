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
    let mut gs = vec![vec![false; n]; n];
    for (a, b) in read!([(Usize1, Usize1); m]) {
        gs[a][b] = true;
        gs[b][a] = true;
    }
    for i in 0..n {
        println!(
            "{}",
            (0..n)
                .filter(|&j| i != j && (0..n).any(|k| gs[i][k] && gs[k][j] && !gs[i][j]))
                .count()
        );
    }
}
