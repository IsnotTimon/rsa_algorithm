mod input;
mod math;
use input::*;
use math::*;

fn main() {
    // N
    println!("Sample RSA starts!");
    println!("Please enter a range to filtrate prime number!");
    let prime_range = input_two();
    if prime_range.len() == 2 {
        let vec_prime = primes_in_range(prime_range[0], prime_range[1]);
        println!("Prime nums in rang:{:?}", vec_prime);
        println!("Please pick two nums.");
    }

    let nums = input_two();
    let (n, p, q) = match nums.as_slice() {
        [p, q] => {
            let n = p * q;
            (n, *p, *q)
        }
        _ => {
            println!("输入有误！");
            return;
        }
    };
    println!("N = {} from: p = {}, q = {}", n, p, q);

    // L
    let l = lcm(p - 1, q - 1);
    println!("L = {}", l);

    // E
    let vec_e = coprimes_in_range(l, 1, l);
    println!("Pick a num as E from:{:?}", vec_e);
    let e = input_single();
    println!("E = {} ", e);

    // D
    let vec_d = find_all_d(e, l);
    println!("Pick a num as D from:{:?}", vec_d);
    let d = input_single();
    println!("D={}", d);

    println!(
        "你的公钥：(E,N)=({},{}) 你的私钥：(D,N)=({},{})",
        e, n, d, n
    );

    // 明文输入
    println!("请输入你要加密的明文.");
    let message = input_single();

    // 加密
    let secret_message = message
        .checked_pow(e as u32)
        .unwrap()
        .checked_rem(n)
        .unwrap();
    println!("明文:{} 秘文:{}", message, secret_message);

    // 解密
    let desecret_message = mod_exp(secret_message, d, n);
    println!("秘文:{} 解密后明文:{} ", secret_message, desecret_message);
}
