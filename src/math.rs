// 最小公倍数
pub fn lcm(a: u64, b: u64) -> u64 {
    (a * b) / gcd(a, b)
}

// 最大公约数
pub fn gcd(mut a: u64, mut b: u64) -> u64 {
    while b != 0 {
        let temp = b;
        b = a % b;
        a = temp;
    }
    a
}

// 挑选范围内素数
pub fn primes_in_range(start: u64, end: u64) -> Vec<u64> {
    (start..=end).filter(|&n| is_prime(n)).collect()
}
// 判断是否是素数
pub fn is_prime(n: u64) -> bool {
    if n <= 1 {
        return false;
    }
    for i in 2..=((n as f64).sqrt() as u64) {
        if n % i == 0 {
            return false;
        }
    }
    true
}

// 找到范围内与给定数字互质的所有数
pub fn coprimes_in_range(num: u64, start: u64, end: u64) -> Vec<u64> {
    (start..=end)
        .filter(|&x| gcd(x, num) == 1) // 互质的定义是 GCD 为 1
        .collect()
}

pub fn find_all_d(e: u64, l: u64) -> Vec<u64> {
    let mut results = Vec::new();

    for d in 2..l {
        if (e * d) % l == 1 {
            results.push(d);
        }
    }

    results
}

// 快速幂取模计算 (a^b) % m
pub fn mod_exp(mut base: u64, mut exp: u64, modulus: u64) -> u64 {
    if modulus == 1 {
        return 0; // 模 1 的结果总是 0
    }
    let mut result = 1;
    base %= modulus; // 预先取模，防止 base 过大

    while exp > 0 {
        // 如果当前位是 1，则累乘结果
        if exp % 2 == 1 {
            result = (result * base) % modulus;
        }
        // 处理下一位：平方 base，右移 exp
        base = (base * base) % modulus;
        exp /= 2;
    }

    result
}
