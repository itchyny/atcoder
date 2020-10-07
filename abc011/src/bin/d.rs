use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (n, d, x, y) = read!((usize, i64, i64, i64));
    if x % d != 0 || y % d != 0 || x.abs() + y.abs() > n as i64 * d {
        println!("0.0");
        return;
    }

    let mut c = vec![vec![0.0; n + 1]; n + 1];
    for i in 0..=n {
        for j in 0..=i {
            if i == 0 {
                c[i][j] = 1.0;
            } else if j == 0 || i == j {
                c[i][j] = c[i - 1][0] / 2.0;
            } else {
                c[i][j] = (c[i - 1][j - 1] + c[i - 1][j]) / 2.0;
            }
        }
    }

    let (x, y) = ((x / d).abs() as usize, (y / d).abs() as usize);
    let p = (x..=x + (n - y - x) / 2)
        .map(|x1| {
            let x2 = x1 - x;
            let y1 = y + (n - x1 - x2 - y) / 2;
            let y2 = y1 - y;
            if x1 + x2 + y1 + y2 == n {
                c[n][x1 + x2] * c[x1 + x2][x1] * c[y1 + y2][y1]
            } else {
                0.0
            }
        })
        .sum::<f64>();
    println!("{}", p);
}
