use rand::{
    Rng,
    distr::{Distribution, Uniform},
};
use rand_distr::{Alphanumeric, StandardUniform};

// 依赖： cargo add rand
fn main() {
    // 1. 生成随机数
    // 通过 rand::thread_rng() 生成随机数，每一个线程都有自己的随机数生成器
    let mut rng = rand::rng();

    let n1: u8 = rng.random();
    let n2: u16 = rng.random();
    println!("Random u8: {}", n1);
    println!("Random u16: {}", n2);
    println!("Random float: {}", rng.random::<f64>());

    // 2. 生成范围内随机数
    println!("Random float: {}", rng.random_range(0.0..1.0)); // 生成范围：[0.0, 1.0)

    let die = Uniform::try_from(1..=6).unwrap();
    loop {
        let throw = die.sample(&mut rng);
        println!("Roll a die: {}", throw);
        if throw == 6 {
            break;
        }
    }

    // 3. 生成多个随机数
    let rand_tuple: (u8, u8, u8) = rng.random();
    println!("Random tuple: {:?}", rand_tuple);

    // 4. 自定义随机类型
    let rand_point = rng.random::<Point>();
    println!("Random point: {:?}", rand_point);

    // 5. 随机密码
    let rand_string: String = rng
        .sample_iter(Alphanumeric)
        .take(30)
        .map(char::from)
        .collect();
    println!("Random string: {}", rand_string);

    // 给定字符范围
    let charset: &[u8] =
        b"abcdefghijklmnopqrstuvwxyzABCDEFGHIJKLMNOPQRSTUVWXYZ0123456789)(*&^%$#@!~";
    let pwd_len: usize = 10;
    let mut rng = rand::rng();
    let password: String = (0..pwd_len)
        .map(|_| charset[rng.random_range(0..charset.len())] as char)
        .collect();
    println!("Random password: {}", password);
}

// 实现 Distribution trait
#[derive(Debug)]
#[allow(dead_code)]
struct Point {
    x: i32,
    y: i32,
}

impl Distribution<Point> for StandardUniform {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Point {
        let x = rng.random();
        let y = rng.random();
        Point { x, y }
    }
}
