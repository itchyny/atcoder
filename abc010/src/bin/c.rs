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

    let (txa, tya, txb, tyb, t, v) = read!((f64, f64, f64, f64, f64, f64));
    if read!([(f64, f64)])
        .any(|(x, y)| distance((txa, tya), (x, y)) + distance((x, y), (txb, tyb)) <= t * v)
    {
        println!("YES");
    } else {
        println!("NO");
    }
}

fn distance((x1, y1): (f64, f64), (x2, y2): (f64, f64)) -> f64 {
    ((x1 - x2) * (x1 - x2) + (y1 - y2) * (y1 - y2)).sqrt()
}
