use std::io::{self, Read as _};

macro_rules! matrix(
    ($n:expr, $m:expr, $f:expr) =>
        (Matrix($n, $m, (0..$n).map(|i| (0..$m).map(|j| $f(i, j)).collect()).collect()));
);

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        ([$tt:tt; $n:expr]) => ((0..$n).map(|_| read!($tt)).collect::<Vec<_>>());
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (k, m) = read!((usize, usize));
    let xs = read!([u32; k]);
    let cs = read!([u32; k]);

    let c = matrix!(k, k, |i, j| {
        if i == k - 1 {
            cs[k - 1 - j]
        } else if i + 1 == j {
            !0
        } else {
            0
        }
    });
    let v = matrix!(1, k, |_, j| if j == 0 { !0 } else { 0 });
    let w = matrix!(k, 1, |i, _| xs[i]);

    println!("{}", (v * (c ^ (m - 1)) * w).2[0][0]);
}

#[derive(Debug, Clone, PartialEq, Eq)]
struct Matrix(usize, usize, Vec<Vec<u32>>);

impl std::ops::Mul for Matrix {
    type Output = Self;

    fn mul(self, other: Self) -> Self {
        assert_eq!(self.1, other.0);
        matrix!(self.0, other.1, |i: usize, j: usize| {
            (0..self.1).fold(0, |x, k| x ^ (self.2[i][k] & other.2[k][j]))
        })
    }
}

impl std::ops::BitXor<usize> for Matrix {
    type Output = Self;

    fn bitxor(self, rhs: usize) -> Self {
        assert_eq!(self.0, self.1);
        let mut r = matrix!(self.0, self.0, |i, j| if i == j { !0 } else { 0 });
        let mut s = self;
        let mut n = rhs;
        while n > 0 {
            if n % 2 == 1 {
                r = r * s.clone();
                n -= 1;
            }
            s = s.clone() * s;
            n /= 2;
        }
        r
    }
}
