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

    let (n, m, mut d) = read!((usize, usize, usize));
    let mut xs: Vec<_> = (0..n).collect();
    for a in read!([usize; m]) {
        xs.swap(a - 1, a);
    }

    let mut ys: Vec<_> = (0..n).collect();
    while d > 0 {
        if d % 2 == 1 {
            for y in ys.iter_mut() {
                *y = xs[*y];
            }
            d -= 1;
        }
        let zs = xs.clone();
        for x in xs.iter_mut() {
            *x = zs[*x];
        }
        d /= 2;
    }

    let mut zs: Vec<_> = (0..n).collect();
    for (i, &y) in ys.iter().enumerate() {
        zs[y] = i;
    }

    for z in &zs {
        println!("{}", z + 1);
    }
}
