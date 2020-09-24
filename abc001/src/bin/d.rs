use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)));
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let n = read!(usize);
    let mut xs: Vec<(u64, u64)> = read!([String; n])
        .map(|x| (x[..4].parse().unwrap(), x[5..].parse().unwrap()))
        .map(|(s, e): (u64, u64)| {
            (s / 5 * 5, {
                let e = (e + 4) / 5 * 5;
                if e % 100 == 60 {
                    e + 40
                } else {
                    e
                }
            })
        })
        .collect();
    xs.sort();
    let mut ys = vec![xs[0]];
    for i in 1..n {
        let l = ys.last_mut().unwrap();
        if l.1 < xs[i].0 {
            ys.push(xs[i]);
        } else if l.1 < xs[i].1 {
            l.1 = xs[i].1;
        }
    }
    for &(s, e) in &ys {
        println!("{:04}-{:04}", s, e);
    }
}
