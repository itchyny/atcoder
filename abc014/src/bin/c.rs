use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)));
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let mut xs: Vec<_> = read!([(u64, u64)])
        .flat_map(|(a, b)| vec![(a, 1), (b + 1, -1)])
        .collect();
    xs.sort();
    let mut ys: Vec<_> = xs.iter().map(|x| x.1).collect();
    for i in 1..ys.len() {
        ys[i] += ys[i - 1];
    }
    println!("{}", ys.iter().max().unwrap());
}
