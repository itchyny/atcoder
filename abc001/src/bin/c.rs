use std::io::{self, Read as _};

fn main() {
    let mut input = String::new();
    io::stdin().read_to_string(&mut input).unwrap();
    let mut input = input.split_whitespace();
    macro_rules! read(
        (($($tt:tt),+)) => (($(read!($tt)),*));
        ($ty:ty) => (input.next().unwrap().parse::<$ty>().unwrap());
    );

    let (deg, dis) = read!((u64, u64));
    let v = velocity(dis);
    if v == 0 {
        println!("C 0");
    } else {
        println!("{} {}", direction(deg), v);
    }
}

fn direction(deg: u64) -> String {
    let xs = vec![
        112, 337, 562, 787, 1012, 1237, 1462, 1687, 1912, 2137, 2362, 2587, 2812, 3037, 3262, 3487,
    ];
    let ys = vec![
        "N", "NNE", "NE", "ENE", "E", "ESE", "SE", "SSE", "S", "SSW", "SW", "WSW", "W", "WNW",
        "NW", "NNW", "N",
    ];
    ys[xs.binary_search(&deg).unwrap_or_else(|x| x)].to_string()
}

fn velocity(dis: u64) -> usize {
    let xs = vec![2, 15, 33, 54, 79, 107, 138, 171, 207, 244, 284, 326];
    let dis = (dis as f64 / 6.0).round() as u64;
    xs.binary_search(&dis).unwrap_or_else(|x| x)
}
