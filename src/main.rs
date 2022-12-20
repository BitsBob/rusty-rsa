use miller_rabin::is_prime;
use rand::Rng;
//https://en.wikipedia.org/wiki/RSA_(cryptosystem)

fn main() {
    let n: u64 = 0x7FFF_FFFF;
    println!("{:?}", is_prime(&n, 20));
}

//This is temporary for testing, will replace with actuall large prime
//generator soon.
pub fn gen_mersenne_prime() -> u64 {
    let num = rand::thread_rng().gen_range(10, 101);
    let prime = 2^num - 1;
    prime
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
    let p = gen_mersenne_prime();
    let q = gen_mersenne_prime();
    let n = p * q;
    let lam_n = lcm((p - 1).try_into().unwrap(), (q - 1).try_into().unwrap());
    let e = rand::thread_rng().gen_range(1, lam_n + 1);
    let d:u32;
}