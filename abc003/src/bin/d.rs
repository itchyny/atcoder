use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (r, c, x, y, d, l) = read!((usize, usize, usize, usize, usize, usize));

    let mut binom = vec![vec![0; x * y + 1]; x * y + 1];
    for i in 0..=x * y {
        for j in 0..=i {
            if i == 0 || j == 0 || i == j {
                binom[i][j] = 1;
            } else {
                binom[i][j] = (binom[i - 1][j - 1] + binom[i - 1][j]) % MOD;
            }
        }
    }

    let mut ans = N(0);
    for x0 in 0..2 {
        for x1 in 0..2 {
            for y0 in 0..2 {
                for y1 in 0..2 {
                    if x > x0 + x1 && y > y0 + y1 && (x - x0 - x1) * (y - y0 - y1) >= d + l {
                        let n = N(binom[(x - x0 - x1) * (y - y0 - y1)][d + l]);
                        if (x0 + x1 + y0 + y1) % 2 == 0 {
                            ans += n;
                        } else {
                            ans -= n;
                        }
                    }
                }
            }
        }
    }
    ans *= N(binom[d + l][d]) * N((r - x + 1) as i64) * N((c - y + 1) as i64);

    println!("{}", ans);
}

const MOD: i64 = 1_000_000_007;

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
struct N(i64);

impl std::fmt::Display for N {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        i64::fmt(&self.0, f)
    }
}

impl std::ops::Add for N {
    type Output = Self;
    fn add(self, other: Self) -> Self {
        Self((self.0 + other.0) % MOD)
    }
}

impl std::ops::AddAssign for N {
    fn add_assign(&mut self, other: Self) {
        *self = *self + other;
    }
}

impl std::ops::Sub for N {
    type Output = Self;
    fn sub(self, other: Self) -> Self {
        Self((self.0 - other.0).rem_euclid(MOD))
    }
}

impl std::ops::SubAssign for N {
    fn sub_assign(&mut self, other: Self) {
        *self = *self - other;
    }
}

impl std::ops::Mul for N {
    type Output = Self;
    fn mul(self, other: Self) -> Self {
        Self((self.0 * other.0) % MOD)
    }
}

impl std::ops::MulAssign for N {
    fn mul_assign(&mut self, other: Self) {
        *self = *self * other;
    }
}
