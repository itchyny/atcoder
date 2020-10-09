use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (a, b) = read!(((i64, i64), (i64, i64)));
    let n = read!(usize);
    let xys = read!([(i64, i64); n]);
    let c = (0..n)
        .map(|i| (xys[i], xys[(i + 1) % n]))
        .filter(|&(xy1, xy2)| cross(xy1, xy2, a, b))
        .count();
    println!("{}", c / 2 + 1);
}

fn cross(
    (x1, y1): (i64, i64),
    (x2, y2): (i64, i64),
    (x3, y3): (i64, i64),
    (x4, y4): (i64, i64),
) -> bool {
    let f = |(x1, y1), (x2, y2)| x1 * y2 - y1 * x2 > 0;
    f((x2 - x1, y2 - y1), (x3 - x1, y3 - y1)) ^ f((x2 - x1, y2 - y1), (x4 - x1, y4 - y1))
        && f((x4 - x3, y4 - y3), (x1 - x3, y1 - y3)) ^ f((x4 - x3, y4 - y3), (x2 - x3, y2 - y3))
}
