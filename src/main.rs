use miller_rabin::is_prime;
use rand::Rng;
use num_primes::Generator;
//https://en.wikipedia.org/wiki/RSA_(cryptosystem)

fn main() {
    let n: u64 = 0x7FFF_FFFF;
    println!("{:?}", is_prime(&n, 20));
    let l = Generator::new_prime(512);
    println!("{}, {}", l, is_prime(&l, 16));
}

fn gcd(mut a:u32, mut b:u32) -> u32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a:u32, b:u32) -> u32 {
    let prod: u32 = a * b;
    prod / gcd(a, b)
}

pub fn gen_key() {
    let p = Generator::new_prime(512);
  let q = Generator::new_prime(512);
    let _n = p * q;
    let lam_n = lcm((p - 1).try_into().unwrap(), (q - 1).try_into().unwrap());
    let mut e = rand::thread_rng().gen_range(1, lam_n + 1); //gcd(e, lam_n) == 1
    while gcd(e, lam_n) != 1 {
        e = rand::thread_rng().gen_range(1, lam_n + 1);
    }
    let _d:u32;
}