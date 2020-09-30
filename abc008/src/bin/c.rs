use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt]) => (read!([$tt; read!(usize)]));
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let cs = read!([u64]);
    let ans = cs
        .iter()
        .map(|c| {
            let n = cs.iter().filter(|&d| c % d == 0).count();
            if n % 2 == 0 {
                1.0 / 2.0
            } else {
                (n + 1) as f64 / (2.0 * n as f64)
            }
        })
        .sum::<f64>();
    println!("{}", ans);
}
