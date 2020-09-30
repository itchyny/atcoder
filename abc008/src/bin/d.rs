use std::collections::HashMap;
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

    let (w, h, xys) = read!((u64, u64, [(u64, u64)]));
    println!("{}", solve(1, 1, w, h, &xys, &mut HashMap::new()));
}

fn solve(
    i: u64,
    j: u64,
    k: u64,
    l: u64,
    xys: &Vec<(u64, u64)>,
    dp: &mut HashMap<(u64, u64, u64, u64), u64>,
) -> u64 {
    if let Some(&v) = dp.get(&(i, j, k, l)) {
        return v;
    }
    let v = xys
        .iter()
        .filter(|&&(x, y)| i <= x && x <= k && j <= y && y <= l)
        .map(|&(x, y)| {
            k - i + l - j
                + 1
                + solve(i, j, x - 1, y - 1, xys, dp)
                + solve(i, y + 1, x - 1, l, xys, dp)
                + solve(x + 1, j, k, y - 1, xys, dp)
                + solve(x + 1, y + 1, k, l, xys, dp)
        })
        .max()
        .unwrap_or(0);
    dp.insert((i, j, k, l), v);
    v
}
